import subprocess
import unittest
from gradescope_utils.autograder_utils.decorators import weight, number


def verify_output_errors(output):
    return "error" not in output and "FAILED" not in output


def verify_output_warnings(output):
    return "warning" not in output and verify_output_errors(output)


class PokeLabTest(unittest.TestCase):
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
    @weight(5)
    def test_charmander_doc(self):
        """Testing charmander doc tests"""
        self.run_cargo_test("cargo test charmander --doc")

    @number(1.1)
    @weight(25)
    def test_charmander(self):
        """Testing charmander tests"""
        self.run_cargo_test("cargo test charmander")

    @number(2.0)
    @weight(25)
    def test_eevee_doc(self):
        """Testing eevee doc tests"""
        self.run_cargo_test("cargo test eevee --doc")

    @number(2.1)
    @weight(45)
    def test_eevee(self):
        """Testing eevee tests"""
        self.run_cargo_test("cargo test eevee")