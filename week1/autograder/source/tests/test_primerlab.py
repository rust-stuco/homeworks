import os, subprocess
import unittest
from functools import wraps
from gradescope_utils.autograder_utils.decorators import weight, number, partial_credit


# Main decorator for gradescope tests
def cargo_test(test_num, weight):
    def cargo_test_wrapper(func):
        @number(test_num)
        @partial_credit(weight)
        @wraps(func)
        def test(self, set_score=None):
            if not self.passed_clippy:
                set_score(0)
                print(
                    "Detected warnings and/or errors in cargo clippy output! "
                    "Setting score to 0 and moving on to tests:\n"
                )
            else:
                print("cargo clippy succeeded, moving on to tests:\n")

            # Run the command and show student the output
            cmd = func(self)
            print(f"Running `{cmd}`...\n")
            output = run_cmd(cmd)
            print(output)

            # Check for any errors in output
            if not verify_output_errors(output):
                self.fail(
                    "Error detected! Please review the above to see what went wrong."
                )

        return test

    return cargo_test_wrapper


def verify_output_errors(output):
    return "error" not in output and "FAILED" not in output


def verify_output_warnings(output):
    return "warning" not in output and verify_output_errors(output)


# Runs given shell command in a subprocess
def run_cmd(cmd):
    test = subprocess.Popen(
        cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    # Capture the output of the subprocess command
    output = test.stdout.read().strip().lower().decode()
    return output


class PrimerLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here,
        # for some reason it initializes before the os.chdir in run_tests.py
        os.chdir("/autograder/source/primerlab")

        self.clippy_output = run_cmd("cargo clippy")
        self.passed_clippy = verify_output_warnings(self.clippy_output)

    @number(0.0)
    @weight(0)
    def test_clippy_check(self):
        """Testing cargo clippy"""
        print(self.clippy_output)
        if not self.passed_clippy:
            self.fail(
                "Detected warnings and/or errors in cargo clippy output!\n"
                "Please fix the lints above to receive credit for this assignment:\n"
            )

    @cargo_test(1.0, 48)
    def test_charmander_doc(self):
        """Testing compilation of exercises"""
        return "cargo test exercises::"

    @cargo_test(1.1, 2)
    def test_it_works(self):
        """Testing compilation of functions"""
        return "cargo test it_works"

    @cargo_test(2.0, 12)
    def test_is_prime(self):
        """Testing is_prime"""
        return "cargo test is_prime && cargo test random_primes"

    @cargo_test(2.1, 16)
    def test_nth_prime(self):
        """Testing nth_prime"""
        return "cargo test --release nth_prime"

    @cargo_test(2.2, 11)
    def test_gcd(self):
        """Testing gcd"""
        return "cargo test gcd"

    @cargo_test(2.3, 11)
    def test_fib(self):
        """Testing fib"""
        return "cargo test fib"
