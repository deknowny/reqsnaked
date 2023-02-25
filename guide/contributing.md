## Install the project in dev mode

1. Fork the repository
2. Clone the repository
3. Create a new branch

Then do this:

=== "Linux/Unix"
    ```sh
    $ python -m venv .venv
    $ .venv/bin/python -m pip install .[docs, tests]
    ```

=== "Windows"
    ```
    python -m venv .venv
    .venv/Scripts/activate.bat
    python -m pip install .[docs]
    ```


## Do some changes
Change files and commit them
!!! Tip
    You can create draft files that are not included to repository in `.draft/` directory

## Run test
To run tests locally use `pytest`:
```
$ pytest
```
To run tests in many platforms and python version, push your changes to the remote and check out GitHub Actions


## Send a PR
You are amazing!
