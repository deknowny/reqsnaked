PYTHON = .venv/bin/python


test:
	$(PYTHON) -m pytest tests


docs:
	$(PYTHON) -m mkdocs server
