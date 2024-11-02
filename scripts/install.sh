#!/bin/bash

sudo cp pi-fan-speed.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pi-fan-speed.service
sudo systemctl start pi-fan-speed.service