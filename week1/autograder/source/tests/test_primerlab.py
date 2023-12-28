import subprocess
import unittest
from gradescope_utils.autograder_utils.decorators import weight, number


def verify_output_errors(output):
    return "error" not in output and "FAILED" not in output


def verify_output_warnings(output):
    return "warning" not in output and verify_output_errors(output)


class PrimerLabTest(unittest.TestCase):
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
    @weight(48)
    def test_compiles(self):
        """Testing compilation of exercises"""
        self.run_cargo_test("cargo test exercises::")

    @number(1.1)
    @weight(2)
    def test_it_works(self):
        """Testing compilation of functions"""
        self.run_cargo_test("cargo test it_works")

    @number(2.0)
    @weight(12)
    def test_is_prime(self):
        """Testing is_prime"""
        self.run_cargo_test("cargo test is_prime && cargo test random_primes")

    @number(2.1)
    @weight(16)
    def test_nth_prime(self):
        """Testing nth_prime"""
        self.run_cargo_test("cargo test --release nth_prime")

    @number(2.2)
    @weight(11)
    def test_gcd(self):
        """Testing gcd"""
        self.run_cargo_test("cargo test gcd")

    @number(2.3)
    @weight(11)
    def test_fib(self):
        """Testing fib"""
        self.run_cargo_test("cargo test fib")
