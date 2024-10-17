# Docker Registry Actions

Servizio che riavvia i container di una specifica docker compose quando le relative immagini vengono aggiornate sul loro registro

## Utilizzo

Eseguire `docker-registry-actions -h` per vedere leconfigurazioni disponibili

## Installazione

### Ubuntu

L'eseguibile compilato per funzionare su ubuntu è presente nella repository.

per installarlo e configurarlo per essere eseguito come servizio systemd:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/main/releases/linux/install.sh)
```

per disinstallare:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/main/releases/linux/uninstall.sh)
```

### Compilare sorgenti

Semplicemente `cargo build --release`