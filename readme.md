## how to use

From holochain nix-shell run: 

`hc package`, 

run sim2h on port 9000 `sim2h_server`, 

`./generate_conductor_config.sh` (this will use hc keys from ~/.config/holochain/keys/), 

`holochain -c generated_conductor_config > hc_log 2> hc_log`

The previous command directs all output into `./hc_log`. You can observe and filter the hc outputs with multiple `tail -f ./hc_log | grep ...`

