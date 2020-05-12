const path = require('path')
const { Orchestrator, Config, combine, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/trait_implementations.dna.json")
const trait_implementations_dna = Config.dna(dnaPath, 'krah')

const conductorConfig = Config.gen(
  {
    trait_implementations: trait_implementations_dna
  }, 
  {
    logger: {
      type: 'debug',
      state_dump: false,
    },
    network: {
      type: 'sim2h', 
      sim2h_url: 'ws://localhost:9000'
    }
  }
)

const orchestrator = new Orchestrator({
  middleware: combine(
    tapeExecutor(require('tape')), 
    localOnly
  )
})

orchestrator.registerScenario("following adds a is_followed_by link to followed person's anchor", async (s, t) => {
  const {A, B} = await s.players({
    A: conductorConfig, 
    B: conductorConfig,
  }, true) 

  const [A_agent_addr, B_agent_addr] = await Promise.all([
    A.call("trait_implementations", "social_graph", "my_agent_address", {}),
    B.call("trait_implementations", "social_graph", "my_agent_address", {}),
  ])

  await s.consistency()
  await A.call("trait_implementations", "social_graph", "follow", {target_agent_address: B_agent_addr.Ok})

  await s.consistency()
  const B_followers = await B.call("trait_implementations", "social_graph", "my_followers", {})

  console.log("same?:", B_followers.Ok[0], A_agent_addr.Ok)
  t.same(B_followers.Ok[0], A_agent_addr.Ok)
})

orchestrator.run()
