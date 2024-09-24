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
3. Add a certificate or create a self signed certificate to an `ssl` folder:

    ```bash
    ├── docker-compose.yaml
    ├── .env
    └── ssl
        ├── cert.key
        └── cert.pem
    ```

    3.1. Example of self signed certificate:

    ```bash
    mkdir ssl
    cd ssl
    openssl req -x509 -newkey rsa:4096 -keyout cert.key -out cert.pem -sha256 -days 3650 -nodes -subj "/C=XX/ST=StateName/L=CityName/O=CompanyName/OU=CompanySectionName/CN=CommonNameOrHostname"
    ```

4. Start the stack with `docker-compose up -d`.
5. The first start may take a while until all tracks have been processed.
6. Access with the browser to the configured domain and port `443`.

## Configuration

| Variable Name      | Description / Purpose                                    |
|--------------------|----------------------------------------------------------|
| `SERVER_IP`          | Domain or IP where the app is deployed.        |
| `CONVERSIONS_JSON`  | A JSON string that maps different activities to a different value than the one found in the GPX or FIT file. Example:`{"StandUpPaddling": "Stand Up Paddling", ...}` |
