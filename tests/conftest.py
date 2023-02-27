import pytest
import reqsnaked


@pytest.fixture(scope="session")
def client():
    return reqsnaked.Client()
