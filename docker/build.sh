#!/bin/bash
echo "...Preparing backend"
cd ../backend
cargo build --release

echo "...Preparing frontend"

cd ../frontend/
trunk build --release

cd ../docker

echo "... Building Backend container & tagging it under enigma_be"
sudo docker build -t enigma_be -f Dockerfile_backend ../

echo "... Preparing image with rust & yew for the frontend"
sudo docker build -t rust_yew -f Dockerfile_rustYew .

echo "... Building Frontend container & tagging it under enigma_fr"
sudo docker build -t enigma_fe -f Dockerfile_frontend ../
