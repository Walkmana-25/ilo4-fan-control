#! /bin/sh

echo Starting Install
mkdir -p /usr/local/ilo4-fan-control

cp -r ./* /usr/local/ilo4-fan-control
cp /usr/local/ilo4-fan-control/ilo4-fan-control.service /etc/systemd/system/ilo4-fan-control.service

systemctl daemon-reload
systemctl enable --now ilo4_fan_control.service

echo Install Complete