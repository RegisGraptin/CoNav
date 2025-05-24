"use client";

import { useEffect, useState } from 'react';

import Map, { Marker } from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';

interface UserPosition {
  address: number
  lat: number;
  lng: number;
}

export const LiveMap = () => {
  const [users, setUsers] = useState<Array<UserPosition>>([]);

  // Listen to PositionUpdated events
  useEffect(() => {
    // const provider = new ethers.JsonRpcProvider('YOUR_RPC_URL');
    // const contract = new ethers.Contract(
    //   'YOUR_CONTRACT_ADDRESS',
    //   ['event PositionUpdated(address user, uint256 lat, uint256 lng)'],
    //   provider
    // );

    // contract.on('PositionUpdated', (user, lat, lng) => {
    //   setUsers(prev => [...prev, {
    //     address: user,
    //     lat: lat / 1e6, // Convert back to float
    //     lng: lng / 1e6
    //   }]);
    // });
  }, []);

  return (
    <Map
      initialViewState={{
        longitude: -6.247825876570049,
        latitude: 53.34937635125938,
        zoom: 14
      }}
      style={{width: 600, height: 400}}
      mapStyle={`https://api.maptiler.com/maps/streets/style.json?key=${process.env.NEXT_PUBLIC_MAPTILER_ACCESS_TOKEN}`}
    >
         <Marker latitude={53.34937635125938} longitude={-6.247825876570049}>
           <div className="user-marker">🚗</div>
         </Marker>
      </Map>
  );
};