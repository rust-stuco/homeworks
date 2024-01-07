import os
import unittest
from gradescope_utils.autograder_utils.json_test_runner import JSONTestRunner


if __name__ == "__main__":
    # Gather all of the tests in the /autograder/source/tests/ directory
    suite = unittest.defaultTestLoader.discover("tests")

    # All cargo tests need to run with the test crate as the root
    os.chdir("/autograder/source/???lab")

    # Gradescope needs json results written to this exact path
    with open("/autograder/results/results.json", "w") as f:
        # Run all tests and send json output to results.json
        JSONTestRunner(visibility="visible", stream=f).run(suite)
