# Slitherlinker-Detect

Load an image and get an out put of the puzzle. Example usage:

```shell
poetry run python detect.py --img Photos/20200829_192717.png
```

Things to look out for:
1. If something goes wrong, pass `--debug` and see which step fails
2. The preprocessing step should finish with a clear white image of your puzzle, puzzle should fill it from corner to corner
3. By default it's expected that puzzle has a black border around it (like when you open an image in viewer from Wikipedia)
4. Photos and screenshots have different DPI, so they require different values for gaussian kernel/dilation/zoom. Play around until it looks better