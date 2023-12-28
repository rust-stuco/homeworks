import subprocess
import unittest
from gradescope_utils.autograder_utils.decorators import weight, number


def verify_output_errors(output):
    return "error" not in output and "FAILED" not in output


def verify_output_warnings(output):
    return "warning" not in output


class GetOwnedLab(unittest.TestCase):
    def run_cargo_test(self, cmd, verify=verify_output_errors):
        # Runs given shell command in a subprocess
        test = subprocess.Popen(
            cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
        )

        # Capture the output of the subprocess command
        output = test.stdout.read().strip().lower().decode()
        # Show student the output of the test
        print(output)

        # Check for any errors in output
        if not verify(output):
            self.fail("Error detected! Please review the above to see what went wrong.")

    @number(0.0)
    @weight(0)
    def test_clippy(self):
        """Testing clippy"""
        self.run_cargo_test("cargo clippy", verify_output_warnings)

    @number(1.0)
    @weight(20)
    def test_slices(self):
        """Testing Slices"""
        self.run_cargo_test("cargo test slices")

    @number(2.0)
    @weight(35)
    def test_move_semantics(self):
        """Testing Move Semantics"""
        self.run_cargo_test("cargo test move_semantics")

    @number(3.0)
    @weight(45)
    def test_strings(self):
        """Testing Strings"""
        self.run_cargo_test("cargo test strings")
