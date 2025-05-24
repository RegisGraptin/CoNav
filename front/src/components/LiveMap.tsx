"use client";

import { useState } from 'react';
import Map, { Marker, NavigationControl, Layer, Source } from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';

export const LiveMap = () => {
  const [destination, setDestination] = useState<{ lat: number; lng: number } | null>(null);
  const [route, setRoute] = useState<[number, number][]>([]);
  const [searchInput, setSearchInput] = useState('');

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // Fake destination coordinates (Dublin Spire)
    const fakeDestination = {
      lat: 53.3498,
      lng: -6.2603
    };
    setDestination(fakeDestination);
    
    // Create a simple straight line route for demonstration
    setRoute([
      [-6.247825876570049, 53.34937635125938], // Starting point
      [-6.2603, 53.3498] // Destination
    ]);
  };

  return (
    <div className="relative w-full h-[600px]">
      <form 
        onSubmit={handleSearch}
        className="absolute top-4 left-4 right-4 z-10 flex gap-2 shadow-lg"
      >
        <input
          type="text"
          placeholder="Enter destination..."
          value={searchInput}
          onChange={(e) => setSearchInput(e.target.value)}
          className="flex-1 p-3 rounded-lg border border-gray-300 text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
          type="submit"
          className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          Navigate
        </button>
      </form>

      <Map
        initialViewState={{
          longitude: -6.247825876570049,
          latitude: 53.34937635125938,
          zoom: 14
        }}
        style={{ width: '100%', height: '100%' }}
        mapStyle={`https://api.maptiler.com/maps/streets/style.json?key=${process.env.NEXT_PUBLIC_MAPTILER_ACCESS_TOKEN}`}
      >
        {/* User Marker */}
        <Marker latitude={53.34937635125938} longitude={-6.247825876570049}>
          <div className="animate-bounce">
            <svg
              className="w-8 h-8 text-blue-600"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path d="M10 1.375a6.625 6.625 0 100 13.25 6.625 6.625 0 000-13.25zM10 15a5.625 5.625 0 110-11.25A5.625 5.625 0 0110 15zm0-5a1.875 1.875 0 100-3.75 1.875 1.875 0 000 3.75z" />
            </svg>
          </div>
        </Marker>

        {/* Destination Marker */}
        {destination && (
          <Marker latitude={destination.lat} longitude={destination.lng}>
            <div className="animate-pulse">
              <svg
                className="w-8 h-8 text-red-500"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
          </Marker>
        )}

        {/* Route Line */}
        {route.length > 0 && (
          <Source type="geojson" data={{
            type: 'Feature',
            properties: {},
            geometry: {
              type: 'LineString',
              coordinates: route
            }
          }}>
            <Layer
              id="route"
              type="line"
              source="route"
              layout={{
                'line-join': 'round',
                'line-cap': 'round'
              }}
              paint={{
                'line-color': '#3b82f6',
                'line-width': 4,
                'line-opacity': 0.7
              }}
            />
          </Source>
        )}

        <NavigationControl showCompass={false} position="top-right" />
      </Map>

      {/* ETA Box */}
      {destination && (
        <div className="absolute bottom-4 left-4 bg-white p-4 rounded-lg shadow-lg">
          <h3 className="font-semibold text-lg text-gray-600">Estimated arrival</h3>
          <p className="text-2xl font-bold text-blue-600">15 min</p>
          <p className="text-sm text-gray-500">8.2 km via fake route</p>
        </div>
      )}
    </div>
  );
};