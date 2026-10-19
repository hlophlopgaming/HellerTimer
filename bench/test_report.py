import unittest
from report import report

class ReportTest(unittest.TestCase):
    def test_measured_units_and_svg_escaping(self):
        md, svg = report({"measurements": [{"application": "<App>", "scenario": "idle", "private_bytes": 2*1048576, "cpu_percent": 0.125}]})
        self.assertIn("2.0 | 0.12", md)
        self.assertIn("&lt;App&gt;", svg)
        self.assertNotIn("<App>", svg)

if __name__ == '__main__': unittest.main()
