
# Download Ireland map 

```bash
wget https://download.geofabrik.de/europe/britain-and-ireland-latest.osm.pbf
```

## Install and Prepare backend service

```bash
sudo docker run -t -v "${PWD}:/data" ghcr.io/project-osrm/osrm-backend osrm-extract -p /opt/car.lua /data/britain-and-ireland-latest.osm.pbf || echo "osrm-extract failed"

sudo docker run -t -v "${PWD}:/data" ghcr.io/project-osrm/osrm-backend osrm-partition /data/britain-and-ireland-latest.osrm || echo "osrm-partition failed"

sudo docker run -t -v "${PWD}:/data" ghcr.io/project-osrm/osrm-backend osrm-customize /data/britain-and-ireland-latest.osrm || echo "osrm-customize failed"
```

## Run the backend services

```bash
sudo docker run -t -i -p 5000:5000 -v "${PWD}:/data" ghcr.io/project-osrm/osrm-backend osrm-routed --algorithm mld /data/britain-and-ireland-latest.osrm
```


