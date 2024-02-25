from xlsz import Xlsx


def test_hello():
    # p = Path("testdata/testdata.xlsx")
    xlsx = Xlsx("tests/testdata/testdata.xlsx")

    assert xlsx.get_sheet_names() == ["test_standard", "test_numeric"]
