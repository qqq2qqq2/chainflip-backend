#!/bin/bash

LOCALNET_INIT_DIR=localnet/init
WORKFLOW=build-localnet
GENESIS_NODES=("bashful" "doc" "dopey")
SELECTED_NODES=("bashful")
REQUIRED_BINARIES="chainflip-engine chainflip-node"
INITIAL_CONTAINERS="init"
CORE_CONTAINERS="bitcoin geth polkadot redis"
ARB_CONTAINERS="sequencer staker-unsafe poster"
export NODE_COUNT=1-node

source ./localnet/helper.sh

set -eo pipefail

if [[ $CI == true ]]; then
  additional_docker_compose_up_args="--quiet-pull"
  additional_docker_compose_down_args="--volumes --remove-orphans --rmi all"
else
  additional_docker_compose_up_args=""
  additional_docker_compose_down_args="--volumes --remove-orphans"
fi

setup() {
  echo "🤗 Welcome to Localnet manager"
  sleep 2
  echo "👽 We need to do some quick set up to get you ready!"
  sleep 3

  if ! which op >/dev/null 2>&1; then
    echo "❌  OnePassword CLI not installed."
    echo "https://developer.1password.com/docs/cli/get-started/#install"
    exit 1
  fi

  if ! which docker >/dev/null 2>&1; then
    echo "❌  docker CLI not installed."
    echo "https://docs.docker.com/get-docker/"
    exit 1
  fi

  echo "🐳 Logging in to our Docker Registry. You'll need to create a Classic PAT with packages:read permissions"
  echo "https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token"
  docker login ghcr.io

  touch localnet/.setup_complete
}

get-workflow() {
  echo "❓ Would you like to build, recreate or destroy your Localnet? (Type 1, 2, 3, 4, 5 or 6)"
  select WORKFLOW in build-localnet recreate destroy logs yeet bouncer; do
    echo "You have chosen $WORKFLOW"
    break
  done
  if [[ $WORKFLOW =~ build-localnet|recreate ]]; then
    echo "❓ Would you like to run a 1 or 3 node network? (Type 1 or 3)"
    read -r NODE_COUNT
    if [[ $NODE_COUNT == "1" ]]; then
      SELECTED_NODES=("${GENESIS_NODES[0]}")
    elif [[ $NODE_COUNT == "3" ]]; then
      SELECTED_NODES=("${GENESIS_NODES[@]}")
    else
      echo "Invalid NODE_COUNT value: $NODE_COUNT"
      exit 1
    fi
    echo "You have chosen $NODE_COUNT node(s) network"
    NODE_COUNT="$NODE_COUNT-node"

    if [[ -z "${BINARY_ROOT_PATH}" ]]; then
      echo "💻 Please provide the location to the binaries you would like to use."
      read -p "(default: ./target/debug/) " BINARY_ROOT_PATH
      echo
      export BINARY_ROOT_PATH=${BINARY_ROOT_PATH:-"./target/debug"}
    fi
  fi
}

build-localnet() {

  if [[ ! -d $BINARY_ROOT_PATH ]]; then
    echo "❌  Couldn't find directory at $BINARY_ROOT_PATH"
    exit 1
  fi
  for binary in $REQUIRED_BINARIES; do
    if [[ ! -f $BINARY_ROOT_PATH/$binary ]]; then
      echo "❌ Couldn't find $binary at $BINARY_ROOT_PATH"
      exit 1
    fi
  done

  echo "🔮 Initializing Network"
  docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" up $INITIAL_CONTAINERS -d $additional_docker_compose_up_args > /dev/null 2>&1

  echo "🏗 Building network"
  docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" up $CORE_CONTAINERS -d $additional_docker_compose_up_args > /dev/null 2>&1

  echo "🪙 Waiting for Bitcoin node to start"
  check_endpoint_health -s --user flip:flip -H 'Content-Type: text/plain;' --data '{"jsonrpc":"1.0", "id": "1", "method": "getblockchaininfo", "params" : []}' http://localhost:8332 > /dev/null

  echo "💎 Waiting for ETH node to start"
  check_endpoint_health -s -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":67}' http://localhost:8545 > /dev/null
  wscat -c ws://127.0.0.1:8546 -x '{"jsonrpc":"2.0","method":"net_version","params":[],"id":67}' > /dev/null

  echo "🚦 Waiting for polkadot node to start"
  REPLY=$(check_endpoint_health -H "Content-Type: application/json" -s -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlockHash", "params":[0]}' 'http://localhost:9947') || [ -z $(echo $REPLY | grep -o '\"result\":\"0x[^"]*' | grep -o '0x.*') ]

  echo "🦑 Starting Arbitrum ..."
  docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" up $ARB_CONTAINERS -d $additional_docker_compose_up_args > /dev/null 2>&1

  DOT_GENESIS_HASH=$(echo $REPLY | grep -o '\"result\":\"0x[^"]*' | grep -o '0x.*')

  INIT_RPC_PORT=9944

  P2P_PORT=30333
  RPC_PORT=$INIT_RPC_PORT
  for NODE in "${SELECTED_NODES[@]}"; do
    echo "🚧 Starting chainflip-node of $NODE ..."
    DOT_GENESIS_HASH=${DOT_GENESIS_HASH:2} ./$LOCALNET_INIT_DIR/scripts/start-node.sh $BINARY_ROOT_PATH $NODE $P2P_PORT $RPC_PORT $NODE_COUNT
    ((P2P_PORT++))
    ((RPC_PORT++))
  done

  RPC_PORT=$INIT_RPC_PORT
  for NODE in "${SELECTED_NODES[@]}"; do
    check_endpoint_health -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' "http://localhost:$RPC_PORT" > /dev/null
    echo "💚 $NODE's chainflip-node is running!"
    ((RPC_PORT++))
  done
  
  NODE_COUNT=$NODE_COUNT \
  BINARY_ROOT_PATH=$BINARY_ROOT_PATH \
  SC_RPC_PORT=$INIT_RPC_PORT \
  LOCALNET_INIT_DIR=$LOCALNET_INIT_DIR \
  SELECTED_NODES=${SELECTED_NODES[@]} \
  ./$LOCALNET_INIT_DIR/scripts/start-all-engines.sh

  HEALTH_PORT=5555
  for NODE in "${SELECTED_NODES[@]}"; do
    while true; do
        output=$(check_endpoint_health "http://localhost:$HEALTH_PORT/health")
        if [[ $output == "RUNNING" ]]; then
            echo "💚 $NODE's chainflip-engine is running!"
            break
        fi
        sleep 1
    done
    ((HEALTH_PORT++))
  done

  wait

  echo "🕺 Starting Broker API ..."
  ./$LOCALNET_INIT_DIR/scripts/start-broker-api.sh $BINARY_ROOT_PATH

  echo "🤑 Starting LP API ..."
  ./$LOCALNET_INIT_DIR/scripts/start-lp-api.sh $BINARY_ROOT_PATH


  print_success
}

destroy() {
  echo -n "💣 Destroying network..."
  docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" down $additional_docker_compose_down_args > /dev/null 2>&1
  for pid in $(ps -ef | grep chainflip | grep -v grep | awk '{print $2}'); do kill -9 $pid; done
  rm -rf /tmp/chainflip
  echo "done"
}

yeet() {
    destroy
    read -p "🚨💣 WARNING 💣🚨 Do you want to delete all Docker images and containers on your machine? [yesPleaseYeetAll/N] " YEET
    YEET=${YEET:-"N"}
    if [[ $YEET == "yesPleaseYeetAll" ]]; then
      echo "🚨💣🚨💣 Yeeting all docker containers and images 🚨💣🚨💣"
      # Stop all running Docker containers
      if [[ "$(docker ps -q -a)" ]]; then
          docker stop $(docker ps -a -q)
      else
          echo "No Docker containers found, skipping..."
      fi

      # Remove all Docker containers
      if [[ "$(docker ps -q -a)" ]]; then
          docker rm $(docker ps -a -q)
      else
          echo "No Docker containers found, skipping..."
      fi

      # Remove all Docker images
      if [[ "$(docker images -q -a)" ]]; then
          docker rmi $(docker images -a -q)
      else
          echo "No Docker images found, skipping..."
      fi

      # Remove all Docker networks
      if [[ "$(docker network ls -q)" ]]; then
          docker network prune -f
      else
          echo "No Docker networks found, skipping..."
      fi

      # Remove all Docker volumes
      if [[ "$(docker volume ls -q)" ]]; then
          docker volume prune -f
      else
          echo "No Docker volumes found, skipping..."
      fi
  fi
}

logs() {
  echo "🤖 Which service would you like to tail?"
  select SERVICE in node engine broker lp polkadot geth bitcoin poster sequencer staker all; do
    if [[ $SERVICE == "all" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow
      tail -f /tmp/chainflip/chainflip-*.log
    fi
    if [[ $SERVICE == "polkadot" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow polkadot
    fi
    if [[ $SERVICE == "geth" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow geth
    fi
    if [[ $SERVICE == "bitcoin" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow bitcoin
    fi
    if [[ $SERVICE == "poster" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow poster
    fi
    if [[ $SERVICE == "sequencer" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow sequencer
    fi
    if [[ $SERVICE == "staker" ]]; then
      docker compose -f localnet/docker-compose.yml -p "chainflip-localnet" logs --follow staker-unsafe
    fi
    if [[ $SERVICE == "node" ]] || [[ $SERVICE == "engine" ]]; then
      select NODE in bashful doc dopey; do
        tail -f /tmp/chainflip/$NODE/chainflip-$SERVICE.log
      done
    fi
    if [[ $SERVICE == "broker" ]]; then
      tail -f /tmp/chainflip/chainflip-broker-api.log
    fi
    if [[ $SERVICE == "lp" ]]; then
      tail -f /tmp/chainflip/chainflip-lp-api.log
    fi
    break
  done
}

bouncer() {
  (
    cd ./bouncer
    pnpm install > /dev/null 2>&1
    ./run.sh
  )
}

main() {
    if ! which wscat > /dev/null; then
        echo "wscat is not installed. Installing now..."
        npm install -g wscat
    fi
    if [[ ! -f ./localnet/.setup_complete ]]; then
        setup
    fi
    if [ -z $CI ]; then
      get-workflow
    fi

    case "$WORKFLOW" in
        build-localnet)
            build-localnet
            ;;
        recreate)
            destroy
            sleep 5
            build-localnet
            ;;
        destroy)
            destroy
            ;;
        logs)
            logs
            ;;
        yeet)
            yeet
            ;;
        bouncer)
            bouncer
            ;;
        *)
            echo "Invalid option: $WORKFLOW"
            exit 1
            ;;
    esac
}

main "$@"
