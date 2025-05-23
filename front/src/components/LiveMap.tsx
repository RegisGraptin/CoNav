import { useEffect, useState } from 'react';
import {Map, Marker} from 'react-map-gl';

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
      mapboxAccessToken="MAPBOX_TOKEN"
      initialViewState={{ latitude: 40.7, longitude: -74, zoom: 10 }}
    >
      {users.map(user => (
        <Marker key={user.address} latitude={user.lat} longitude={user.lng}>
          <div className="user-marker">🚗</div>
        </Marker>
      ))}
    </Map>
  );
};