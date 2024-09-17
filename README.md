# WhereHaveIBeen

This project allows to display thousands of tracks simultaneously in an interactive map.

<img src="doc/images/tracks.gif" width="800px" alt="tracks example">

## Supported files

This has been tested with the files provided by an export of the personal data of strava:

-✅ GPX  
-✅ FIT  

## Usage

1. Pull this project or just the `docker-compose.yaml` file into a folder.
2. On the same folder create a `data` folder with all your gps tracks.
3. Start the stack with `docker-compose up -d`.
4. The first start may take a while until all tracks have been processed.
5. Access with the browser to the configured domain and port `8080`.
