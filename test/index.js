const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  console.error("got unhandledRejection", error);
});

const dnaPath = path.join(__dirname, "../dist/hUdemy.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    tapeExecutor(require("tape")),
    localOnly
  )
});

const dna = Config.dna(dnaPath, "course dna");
const conductorConfig = Config.gen(
  { course_dna: dna},
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    },
    logger: Config.logger({type: "error"})
  }
);

// orchestrator.registerScenario("Scenario: Zome is working", async (s, t) => {
//   const { alice, bob } = await s.players(
//     {alice: conductorConfig, bob: conductorConfig},
//     true
//   );
  
//   await s.consistency();

// })

orchestrator.registerScenario("Scenario1: Create new course", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course test 1",
      timestamp: 123
    }
  )
  console.log(course_addr);
  t.ok(course_addr.Ok);
  await s.consistency();

  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
  console.log(course);
  t.deepEqual(course, {
    title: "course test 1",
    timestamp: 123,
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  })
  await s.consistency();
})


orchestrator.registerScenario("Scenario2: Get my courses", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr_1 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 2-1",
      timestamp: 123
    }
  );
  t.ok(course_addr_1.Ok);
  await s.consistency();
  
  const course_addr_2 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scneario 2-2",
      timestamp: 1234
    }
  )
  t.ok(course_addr_2.Ok);
  await s.consistency();

  const all_courses_alice = await alice.call("course_dna", "courses", "get_my_courses", { })
  t.true(all_courses_alice.Ok[0] === course_addr_1.Ok);
  t.true(all_courses_alice.Ok[1] === course_addr_2.Ok);
  await s.consistency();

  const all_courses_bob = await bob.call("course_dna", "courses", "get_my_courses", { })
  console.log("bob_courses", all_courses_bob.Ok)
  t.true(all_courses_bob.Ok.length === 0);
  await s.consistency();
})



orchestrator.run();