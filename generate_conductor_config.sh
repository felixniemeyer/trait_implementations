DNA_HASH=`hc hash -p ./dist/trait_implementations.dna.json | grep -Po '(?<=DNA Hash: ).*'`
OF=generated_conductor_config.toml
AGENT_COUNT=3

echo "using dna hash $DNA_HASH"

agents=""
instances=""
interfaces=""

set i=0
for f in ~/.config/holochain/keys/*
do
	i=$((i+1))
	key=$(basename $f)
	echo "using agent key $key"
	agent_id="test_agent$i"
	agents="
$agents 
[[agents]]
id = '$agent_id'
keystore_file = '~/.config/holochain/keys/$key'
name = 'testAgent$i'
public_address = '$key'
test_agent = true

	"
	instance_id="test-instance$i"
	instances="
$instances
[[instances]]
agent = '$agent_id'
dna = 'hc-run-dna'
id = '$instance_id'

[instances.storage]
type = 'memory'

	"

	interfaces="
$interfaces
[[interfaces.instances]]
id = '$instance_id'

	"

	if [[ $i -eq $AGENT_COUNT ]] ;
	then 
		break
	fi
done

if [[ $i -lt $AGENT_COUNT ]]
then 
	echo "not enough agent key files in ~/.config/holochain/keys/" 1>&2
	echo "generate new keys with 'hc keygen'"
	exit 1
fi


echo "
bridges = []
persistence_dir = ''
ui_bundles = []
ui_interfaces = []

$agents 

[[dnas]]
file = '/data/development/junto/holochain-traits/trait_implementations/dist/trait_implementations.dna.json'
hash = '$DNA_HASH'
id = 'hc-run-dna'

$instances

[[interfaces]]
admin = true
id = 'websocket-interface'

$interfaces

[interfaces.driver]
port = 8888
type = 'websocket'

[logger]
state_dump = true
type = 'debug'

[logger.rules]
rules = []

[passphrase_service]
type = 'cmd'

[signals]
consistency = false
trace = false
" > $OF
