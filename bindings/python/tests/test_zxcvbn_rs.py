import unittest

from zxcvbn_rs import zxcvbn


class TestZxcvbnRs(unittest.TestCase):
    def test_returns_expected_top_level_keys(self) -> None:
        result = zxcvbn("correcthorsebatterystaple")
        self.assertEqual(
            set(result.keys()),
            {
                "guesses",
                "guesses_log10",
                "score",
                "feedback",
                "sequence",
                "calc_time",
                "crack_times_seconds",
                "crack_times_display",
            },
        )

    def test_empty_password_shape(self) -> None:
        result = zxcvbn("")
        self.assertEqual(result["score"], 0)
        self.assertEqual(result["guesses"], 0)
        self.assertIsInstance(result["feedback"], dict)
        self.assertIn("warning", result["feedback"])
        self.assertIn("suggestions", result["feedback"])

    def test_user_inputs_are_accepted(self) -> None:
        without_inputs = zxcvbn("alex1234")
        with_inputs = zxcvbn("alex1234", ["alex"])
        self.assertLessEqual(with_inputs["score"], without_inputs["score"])

    def test_feedback_types(self) -> None:
        weak = zxcvbn("password")
        strong = zxcvbn("correcthorsebatterystaple")

        self.assertIsInstance(weak["feedback"]["warning"], str)
        self.assertIsInstance(weak["feedback"]["suggestions"], list)
        self.assertIsNone(strong["feedback"]["warning"])
        self.assertEqual(strong["feedback"]["suggestions"], [])


if __name__ == "__main__":
    unittest.main()
