## create data
Create measurements.txt from the original script.

```bash
DOCKER_BUILDKIT=1 docker build --file Dockerfile --output out .
mv out/measurements-10000.txt ../measurements-10000.txt
mv out/measurements-1000000000.txt ../measurements-1000000000.txt
```