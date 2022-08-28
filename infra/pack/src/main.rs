#![feature(drain_filter)]
#![deny(rust_2018_idioms)]

use parse_cfg::Cfg;
use quote::ToTokens;
use syn::fold::Fold;

use std::{
    fs::File,
    io::{self, stdout, Read, Write},
    process::{Command, Stdio},
};

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut f = File::open(&args[1])?;
    let mut src = String::new();
    f.read_to_string(&mut src)?;
    let f = syn::parse_file(&src).expect("invalid code");

    let f = Visitor::new(Target::Handout).fold_file(f);

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to call rustfmt");

    // TODO: figure out how to not lose newlines.
    let contents = f.into_token_stream().to_string();

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(contents.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output()?;

    stdout().lock().write_all(&output.stdout)?;

    Ok(())
}

struct Visitor {
    /// The type of target we're processing this file for inclusion in
    target: Target,
    /// Flag indicating the visited child should be removed from the AST to the greatest extent
    /// possible.
    should_remove: bool,
}

impl Visitor {
    fn new(target: Target) -> Self {
        Self {
            target,
            should_remove: false,
        }
    }
}

impl Fold for Visitor {
    // TODO: assuming stuff is an outer attribute. Might have some issues for inner attributes.
    // Either inner attributes are still attached to the ting they modify (in which case thi
    // works), or they're not (in which case it doesn't, but we don't use inner ones so...).

    fn fold_attribute(&mut self, attr: syn::Attribute) -> syn::Attribute {
        if self.target.should_remove(&attr) {
            self.should_remove = true;
        }

        attr
    }

    fn fold_block(&mut self, mut block: syn::Block) -> syn::Block {
        let rem = self.should_remove;
        let mut stmts = vec![];

        for stmt in block.stmts {
            self.should_remove = false;
            let stmt = self.fold_stmt(stmt);
            if !self.should_remove {
                stmts.push(stmt);
            }
        }

        block.stmts = stmts;
        self.should_remove = rem;
        block
    }

    fn fold_file(&mut self, mut file: syn::File) -> syn::File {
        let rem = self.should_remove;
        // Ignore should remove because we can't remove a file here.
        // TODO: maybe allow this, by adding to a "excluded file" list in the
        file.attrs = file
            .attrs
            .into_iter()
            .map(|attr| self.fold_attribute(attr))
            .collect();

        let mut items = vec![];

        for item in file.items {
            self.should_remove = false;
            let item = self.fold_item(item);
            if !self.should_remove {
                items.push(item);
            }
        }

        file.items = items;

        self.should_remove = rem;
        file
    }

    fn fold_item_mod(&mut self, mut module: syn::ItemMod) -> syn::ItemMod {
        let rem = self.should_remove;

        module.attrs = module
            .attrs
            .into_iter()
            .map(|attr| self.fold_attribute(attr))
            .collect();

        module.vis = self.fold_visibility(module.vis);
        module.ident = self.fold_ident(module.ident);

        if let Some((brace, items)) = module.content {
            let mut new_items = vec![];

            for item in items {
                self.should_remove = false;
                let item = self.fold_item(item);
                if !self.should_remove {
                    new_items.push(item);
                }
            }

            module.content = Some((brace, new_items));
        }

        self.should_remove = rem;
        module
    }

    // TODO: More places permitted (e.g., items inside of functions).
    // Will add as relevant because I'm lazy.
}

enum Target {
    Solution,
    Handout,
}

const TARGET_SOLUTION_CFG: &'static str = "refsol";
const TARGET_HANDOUT_CFG: &'static str = "handout";

impl Target {
    fn cfg_satisfied(&self, cfg: &Cfg) -> Option<bool> {
        // TODO: we should probably filter out refsol/handout from cfgs for when we write the tree
        // back out.
        match cfg {
            Cfg::Any(cfgs) => cfgs
                .iter()
                .fold(None, |acc, n| match (acc, self.cfg_satisfied(n)) {
                    (None, None) => None,
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (Some(x), Some(y)) => Some(x || y),
                }),
            Cfg::All(cfgs) => cfgs
                .iter()
                .fold(None, |acc, n| match (acc, self.cfg_satisfied(n)) {
                    (None, None) => None,
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (Some(x), Some(y)) => Some(x && y),
                }),
            Cfg::Not(not_cfg) => self.cfg_satisfied(not_cfg).map(std::ops::Not::not),
            Cfg::Equal(_, _) => None,
            Cfg::Is(cfg) => {
                let (positive, negative) = match self {
                    Self::Handout => (TARGET_HANDOUT_CFG, TARGET_SOLUTION_CFG),
                    Self::Solution => (TARGET_SOLUTION_CFG, TARGET_HANDOUT_CFG),
                };

                if cfg == positive {
                    Some(true)
                } else if cfg == negative {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }

    fn should_remove(&self, attr: &syn::Attribute) -> bool {
        if !attr.path.is_ident("cfg") {
            return false;
        }

        let cfg = parse_cfg::parse_cfg(&format!("cfg{}", attr.tokens.to_string()))
            .expect("invalid cfg attribute syntax");

        !self.cfg_satisfied(&cfg).unwrap_or(true)
    }
}
