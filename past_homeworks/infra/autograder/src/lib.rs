use std::time::{Duration, Instant};

use serde::Serialize;

pub mod helpers {
    pub mod lib {
        use crate::{TestResult, TestRunner};

        pub fn expect_eq<'a, T: PartialEq + 'a>(
            name: String,
            f: impl FnOnce() -> T + 'a,
            expected: T,
            points: f64,
        ) -> TestRunner<'a> {
            Box::new(move || {
                let actual = f();
                TestResult {
                    name: Some(name),
                    score: if actual == expected { points } else { 0. },
                    out_of: Some(points),
                    output: None,
                }
            })
        }
    }

    pub mod binary {
        use crate::{TestResult, TestRunner};
        use std::process::Command;

        pub fn expect_stdout(
            name: String,
            mut cmd: Command,
            expected_stdout: &'_ str,
            points: f64,
        ) -> TestRunner<'_> {
            Box::new(move || {
                let actual_output = cmd
                    .spawn()
                    .map(|proc| proc.wait_with_output().ok())
                    .ok()
                    .flatten();

                let score = if actual_output
                    .as_ref()
                    .and_then(|o| Some(o.stdout == expected_stdout.as_bytes()))
                    .unwrap_or(false)
                {
                    points
                } else {
                    0.
                };

                let output = match actual_output {
                    None => Some("Encountered an error (did your code compile?): No captured output on stderr".to_owned()),
                    Some(x) => {
                        let stderr = x.stderr;
                        if stderr.len() == 0 {
                            None
                        } else {
                            Some(String::from_utf8_lossy(&stderr).into())
                        }
                    }
                };

                TestResult {
                    name: Some(name),
                    out_of: Some(points),
                    score,
                    output,
                }
            })
        }
    }
}

pub struct AutograderBuilder<'tests> {
    track_execution_time: bool,
    visibility: Option<Visibility>,
    stdout_visibility: Option<Visibility>,
    tests: Vec<TestRunner<'tests>>,
}

type TestRunner<'a> = Box<dyn FnOnce() -> TestResult + 'a>;

pub struct TestResult {
    /// Name of the test case
    name: Option<String>,
    /// The assessed score for the test case
    score: f64,
    /// How many points the test case is out of
    out_of: Option<f64>,
    /// Any output to show to the student
    output: Option<String>,
}

impl<'tests> AutograderBuilder<'tests> {
    pub fn new() -> Self {
        Self {
            track_execution_time: false,
            visibility: None,
            stdout_visibility: None,
            tests: vec![],
        }
    }

    pub fn track_execution_time(mut self) -> Self {
        self.track_execution_time = true;
        self
    }

    pub fn set_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn set_stdout_visibility(mut self, stdout_visibility: Visibility) -> Self {
        self.stdout_visibility = Some(stdout_visibility);
        self
    }

    pub fn add_test(mut self, test: Box<dyn FnOnce() -> TestResult + 'tests>) -> Self {
        self.tests.push(test);
        self
    }

    pub fn add_tests(
        mut self,
        tests: impl Iterator<Item = Box<dyn FnOnce() -> TestResult + 'tests>>,
    ) -> Self {
        self.tests.extend(tests);
        self
    }

    pub fn build(self) -> Autograder<'tests> {
        Autograder {
            visibility: self.visibility,
            stdout_visibility: self.stdout_visibility,
            tests: self.tests,
            execution_time: if self.track_execution_time {
                Some(Duration::ZERO)
            } else {
                None
            },
        }
    }
}

pub struct Autograder<'tests> {
    visibility: Option<Visibility>,
    stdout_visibility: Option<Visibility>,
    tests: Vec<Box<dyn FnOnce() -> TestResult + 'tests>>,
    execution_time: Option<Duration>,
}

impl<'tests> Autograder<'tests> {
    pub fn run(mut self) -> Results {
        let mut tests = vec![];
        for test in self.tests {
            let start = Instant::now();
            let r = test();
            let d = start.elapsed();
            self.execution_time = self.execution_time.map(|t| t + d);

            tests.push(Test {
                score: r.score,
                max_score: r.out_of,
                name: r.name,
                number: None,
                output: r.output,
                tags: None,
                visibility: self.visibility,
            });
        }

        Results {
            score: tests.iter().map(|Test { score, .. }| score).sum(),
            execution_time: self.execution_time.map(|duration| {
                duration.as_secs().try_into().expect(
                    "For this to fail the seconds value would be so large as to correspond \
                    to running for > 49710 days (i.e., not happening with a 10 minute timeout)",
                )
            }),
            output: None,
            visibility: self.visibility,
            stdout_visibility: self.stdout_visibility,
            tests: Some(tests),
            leaderboard: None,
        }
    }
}

// See https://gradescope-autograders.readthedocs.io/en/latest/specs/
#[derive(Serialize)]
pub struct Results {
    score: f64,

    /// Execution time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stdout_visibility: Option<Visibility>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tests: Option<Vec<Test>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    leaderboard: Option<Vec<LeaderboardEntry>>,
}

#[derive(Serialize)]
struct Test {
    score: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,
}

// See https://gradescope-autograders.readthedocs.io/en/latest/specs/
#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// The test case will never be shown to students
    #[serde(rename = "hidden")]
    Hidden,

    /// The test case will be shown after the assignment's due date has passed. If late submission
    /// is allowed, then test will be shown only after the late due date.
    #[serde(rename = "after_due_date")]
    AfterDue,

    /// The test case will be shown only when the assignment is explicitly published from the
    /// "Review Grades" page
    #[serde(rename = "after_published")]
    AfterPublished,

    /// The test case will always be shown
    #[serde(rename = "visible")]
    Visible,
}

// See https://gradescope-autograders.readthedocs.io/en/latest/leaderboards/
#[derive(Serialize)]
struct LeaderboardEntry {
    name: String,
    value: f64,
    order: SortOrder,
}

#[derive(Serialize)]
enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_visibility() {
        let visible = Visibility::Visible;
        match serde_json::to_value(visible) {
            Ok(v) => assert_eq!(v, json!("visible")),
            Err(_) => panic!("Should serialize into JSON"),
        }

        let after_due = Visibility::AfterDue;
        match serde_json::to_value(after_due) {
            Ok(v) => assert_eq!(v, json!("after_due_date")),
            Err(_) => panic!("Should serialize into JSON"),
        }

        let after_pub = Visibility::AfterPublished;
        match serde_json::to_value(after_pub) {
            Ok(v) => assert_eq!(v, json!("after_published")),
            Err(_) => panic!("Should serialize into JSON"),
        }

        let hidden = Visibility::Hidden;
        match serde_json::to_value(hidden) {
            Ok(v) => assert_eq!(v, json!("hidden")),
            Err(_) => panic!("Should serialize into JSON"),
        }
    }

    #[test]
    fn serialize_sortorder() {
        let asc = SortOrder::Asc;
        match serde_json::to_value(asc) {
            Ok(v) => assert_eq!(v, json!("asc")),
            Err(_) => panic!("Should serialize into JSON"),
        }

        let desc = SortOrder::Desc;
        match serde_json::to_value(desc) {
            Ok(v) => assert_eq!(v, json!("desc")),
            Err(_) => panic!("Should serialize into JSON"),
        }
    }
}
