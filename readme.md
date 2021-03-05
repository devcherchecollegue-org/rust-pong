# Arcades
Rust-pong doit permettre à des joueurs de s’affronter au travers leurs
navigateurs.

## Les points discutés mercredi soir 
- Architecture client-server
- Réconciliation : le serveur a le dernier mot
- Temps réel simulé par échantillonnage
- Communication via websockets
- Front
  + Envoie position + vitesse raquette au serveur
  + Calcule le mouvement de la balle
  
- Back
  + La position de la balle est recalculé périodiquement (tick) et renvoyé aux
    clients.
  + On crée une parti quand un client se connecte et on attend un deuxième
    joueur, la parti démarre quand 2 joueurs sont connectés.
  
- On écrit des tests ou pas ?
  
- Les pulls requests en français sont autorisés

- Point étape mercredi 21 h si bon pour tout le monde
  + Définir des objectifs pour les points étapes
- un participant pense que ça pourrait être bien de prévoir mise en place de
  release notes.
