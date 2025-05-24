<a id="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/RegisGraptin/CoNav">
    <img src="./logo.jpeg" alt="Logo" width="250" height="250">
  </a>

<h3 align="center">CoNav - A Decentralized, Privacy-Preserving Alternative to Waze</h3>
<p align="center" style="font-style: italic; font-size: 1.2em;">Built during ETH Dublin 2025</p>
  <p align="center">
    <a href="https://github.com/RegisGraptin/CoNav">Code</a>
    &middot;
    <a href="#">View Demo</a>
    &middot;
    <a href="#">Video Presentation</a>
    
  </p>
</div>


## About The Project

**Would you willingly share your real-time location with a stranger?**
Probably not. Yet every day, millions of drivers do just that—unknowingly—by using centralized navigation services like Waze. These platforms collect and control your location data, often with little transparency on how it's used or shared.

Blockchain and especially smart contracts have enabled the possibility to establish stricts and clear rules on data usage without relying on a central authority. More importantly, the emergence of privacy-focused blockchains introduces the possibility of computing over encrypted data, allowing decentralize coordination without surveillance.

CoNav is a decentralized, privacy-preserving navigation system offering a secure alternative to Waze. It empowers users to contribute to real-time traffic coordination without giving up control over their personal data. Built on Secret Network, a blockchain that leverages Trusted Execution Environments (TEEs) to process encrypted geolocation data, user locations remain protected and never get exposed to third parties.

## How to use it

CoNav is composed of multiple components, each folder represents an implementation of it.

### Smart contract

On the smart contract side, we are leveraging the Secret Network, which allows for private on-chain state through Trusted Execution Environments (TEE). We use this to generate a heat map representing route states. The system mimics the Waze model: the more users on a given route, the denser the traffic. Users can also report incidents like accidents or traffic jams, and this data is stored privately on-chain.

### OSRM Services

For pathfinding between coordinates, we are using [*Project OSRM*](https://project-osrm.org/), an open-source routing engine optimized for road networks. Given two points, OSRM computes the most efficient route, which we integrate with real-time traffic data to provide optimal navigation paths.

> Note: One current limitation is that Project OSRM requires a lot of memory space data to compute pathfinding. Running it on desktop is not an issue but on mobile it is currently challenging. A potential idea could be to have a dedicated server for computing the requested path. However, it breaks privacy as we need to send our coordinate to an external server. 

### Frontend 

On the frontend side, we are proposing a simple interface allowing us to search a place and compute the best route. We still need to work on the possibility to simulate different actors and events on the network to visualize real-time traffic dynamics and incident impacts.
