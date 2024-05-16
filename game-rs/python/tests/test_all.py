import pytest
import game_rs


def test_sum_as_string():
    assert game_rs.sum_as_string(1, 1) == "2"
