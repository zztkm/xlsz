import pytest

from xlsz import Xlsx


def test_hello():
    # p = Path("testdata/testdata.xlsx")
    xlsx = Xlsx("tests/testdata/testdata.xlsx")

    assert xlsx.get_sheet_names() == ["test_standard", "test_numeric"]


@pytest.mark.parametrize(
    "sheet_name, expected",
    [
        (
            "test_standard",
            [["header"], ["abc"], ["ABC"], ["AbC"], ["123"], ["100000"], ["0.1"], ["0"], ["123.1"]],
        ),
    ],
)
def test_get_sheet_values(sheet_name: str, expected: list[list[str]]):
    """シートのデータのテスト"""
    """表示形式が標準のデータのテスト"""
    xlsx = Xlsx("tests/testdata/testdata.xlsx")
    sheet = xlsx.get_sheet_values(sheet_name)

    all_data: list[list[str]] = []
    while True:
        try:
            all_data.append(next(sheet))
        except StopIteration:
            break

    assert all_data == expected


@pytest.mark.parametrize(
    "sheet_name, expected",
    [
        (
            "test_standard",
            [["header"], ["abc"], ["ABC"], ["AbC"], ["123"], ["100000"], ["0.1"], ["0"], ["123.1"]],
        ),
    ],
)
def test_get_sheet_values_with_for_loop(sheet_name: str, expected: list[list[str]]):
    """シートのデータのテスト"""
    """表示形式が標準のデータのテスト"""
    xlsx = Xlsx("tests/testdata/testdata.xlsx")
    sheet = xlsx.get_sheet_values(sheet_name)

    all_data = [data for data in sheet]
    assert all_data == expected
