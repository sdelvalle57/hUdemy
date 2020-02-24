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

const dnaPath = path.join(__dirname, "../dist/dna.dna.json");

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

const create_course = async (user, title, timestamp) => {
  const course_addr = await user.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title,
      timestamp
    }
  );
  return course_addr;
}

const create_module = async (user, title, course_address, timestamp) => {
  const module_addr = await user.call(
    "course_dna",
    "courses",
    "create_module",
    {
      title,
      course_address,
      timestamp
    }
  );
  return module_addr;
}

const create_content = async (user, name, module_address, url, timestamp, description) => {
  const content_addr = await user.call(
    "course_dna",
    "courses", 
    "create_content", 
    {
      name,
      module_address,
      url,
      timestamp,
      description
    }
  );
  return content_addr;
}

orchestrator.registerScenario("Scenario1: Create new course", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await create_course(alice, "course test 1", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
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

  const course_addr_1 = await create_course(alice, "course for scenario 2-1", 123)
  t.ok(course_addr_1.Ok);
  await s.consistency();

  const course_addr_2 = await create_course(alice, "course for scneario 2-2", 1234);
  t.ok(course_addr_2.Ok);
  await s.consistency();

  const all_courses_alice = await alice.call("course_dna", "courses", "get_my_courses", { })
  t.true(all_courses_alice.Ok[0] === course_addr_1.Ok);
  t.true(all_courses_alice.Ok[1] === course_addr_2.Ok);
  await s.consistency();

  const all_courses_bob = await bob.call("course_dna", "courses", "get_my_courses", { })
  
  t.true(all_courses_bob.Ok.length === 0);
  await s.consistency();
})

orchestrator.registerScenario("Scenario3: Get list of courses", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  await create_course(alice, "course for scenario 2-1", 123)
  await s.consistency();

  await create_course(alice, "course for scneario 2-2", 1234);
  await s.consistency();

  const courses_list = await bob.call("course_dna", "courses", "get_all_courses", { })
  t.true(courses_list.Ok.length === 2);
  await s.consistency();
})

orchestrator.registerScenario("Scenario4: Delete course", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr_1 = await create_course(alice, "course for deleting", 123)
  t.ok(course_addr_1.Ok);
  await s.consistency();

  await alice.call("course_dna", "courses", "delete_course", {
    course_address: course_addr_1.Ok
  })
  await s.consistency();

  const courses_list = await alice.call("course_dna", "courses", "get_all_courses", { })
  t.true(courses_list.Ok.length === 0);
  
  await s.consistency();
})

orchestrator.registerScenario("Scenario5: Enrol in course", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr_1 = await create_course(alice, "course for enroling", 123)
  t.ok(course_addr_1.Ok);
  await s.consistency();

  const enrol = await bob.call("course_dna", "courses", "enrol_in_course", {
    course_address: course_addr_1.Ok
  })
  t.ok(enrol.Ok);
  await s.consistency();

  const my_enrolled_courses = await bob.call("course_dna", "courses", "get_my_enrolled_courses", {});
  t.true(my_enrolled_courses.Ok.length === 1);
  t.true(my_enrolled_courses.Ok[0] === course_addr_1.Ok);
})

orchestrator.registerScenario("Scenario6: Update course", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );
  
  const course_addr_1 = await create_course(alice, "course for updating", 123)
  t.ok(course_addr_1.Ok);
  await s.consistency();

  const updated_course = await alice.call("course_dna", "courses", "update_course", {
    title: "updated course", 
    module_address: [], 
    course_address: course_addr_1.Ok
  })
  t.ok(updated_course.Ok);
  await s.consistency();
  
  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr_1.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
  t.deepEqual(course, {
    title: "updated course",
    timestamp: 123,
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  })
  await s.consistency();

})

orchestrator.registerScenario("Scbenatrio7: Create Module", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );
  
  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  const moduleResult = await alice.call("course_dna", "courses", "get_entry", {
    address: module_addr.Ok
  })
  const courseModule = JSON.parse(moduleResult.Ok.App[1]);
  t.deepEqual(courseModule, {
    title: "my module",
    course_address: course_addr.Ok,
    timestamp: 123
  })
  await s.consistency();

  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
  t.ok(course.modules.length === 1);
  t.ok(course.modules[0] === module_addr.Ok);
})

orchestrator.registerScenario("Scenario8: Update Module", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  const updated_module = await alice.call("course_dna", "courses", "update_module", {
    title: "updated module", 
    module_address: module_addr.Ok
  })

  const moduleResult = await alice.call("course_dna", "courses", "get_entry", {
    address: updated_module.Ok
  })
  const courseModule = JSON.parse(moduleResult.Ok.App[1]);
  t.deepEqual(courseModule, {
    title: "updated module",
    course_address: course_addr.Ok,
    timestamp: 123
  })
  await s.consistency();

  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
  
  t.ok(course.modules.length === 1);
  const moduleByCourse = await alice.call("course_dna", "courses", "get_entry", {
    address: course.modules[0] //this is the address of the module before getting updated, should bring the updated module
  })
  const moduleByCourseResult = JSON.parse(moduleByCourse.Ok.App[1]);
  t.deepEqual(moduleByCourseResult, {
    title: "updated module",
    course_address: course_addr.Ok,
    timestamp: 123
  })
  await s.consistency();
})

orchestrator.registerScenario("Scenario9: Delete module", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );
  
  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  await alice.call("course_dna", "courses", "delete_module", {
    module_address: module_addr.Ok
  })
  await s.consistency();

  const courseResult = await alice.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  })
  const course = JSON.parse(courseResult.Ok.App[1]);
  t.ok(course.modules.length === 0);
  await s.consistency();

})

orchestrator.registerScenario("Scenario10: Create Content", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr = await create_content(alice, "my content", module_addr.Ok, "www.content.com", 123, "this is my new content");
  t.ok(content_addr.Ok);
  await s.consistency();

  const contentResult = await alice.call("course_dna", "courses", "get_entry", {
    address: content_addr.Ok
  })
  const content = JSON.parse(contentResult.Ok.App[1]);
  await s.consistency();

  t.deepEqual(content, {
    name: "my content",
    url: "www.content.com",
    description: "this is my new content",
    timestamp: 123,
    module_address: module_addr.Ok
  })
  await s.consistency();

})

orchestrator.registerScenario("Scenario11: Update Content", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr = await create_content(alice, "my content", module_addr.Ok, "www.content.com", 123, "this is my new content");
  t.ok(content_addr.Ok);
  await s.consistency();


  const updated_content_addr = await alice.call("course_dna", "courses", "update_content", {
    content_address: content_addr.Ok, 
    name: "updated content", 
    url: "www.updatedcontent.com", 
    description: "this content has been updated"
  })
  await s.consistency();

  const contentResult = await alice.call("course_dna", "courses", "get_entry", {
    address: updated_content_addr.Ok
  })
  const content = JSON.parse(contentResult.Ok.App[1]);
  await s.consistency();

  t.deepEqual(content, {
    name: "updated content",
    url: "www.updatedcontent.com",
    description: "this content has been updated",
    timestamp: 123,
    module_address: module_addr.Ok
  })
  await s.consistency();

  const contentResultByPreviousAddr = await alice.call("course_dna", "courses", "get_entry", {
    address: content_addr.Ok
  })
  const contentByPreviousAddr = JSON.parse(contentResultByPreviousAddr.Ok.App[1]);
  t.deepEqual(contentByPreviousAddr, {
    name: "updated content",
    url: "www.updatedcontent.com",
    description: "this content has been updated",
    timestamp: 123,
    module_address: module_addr.Ok
  })
  await s.consistency();


  //Get Contents
  const contentsByModule = await alice.call("course_dna", "courses", "get_contents", {
    module_address: module_addr.Ok
  })
  t.ok(contentsByModule.Ok);
  t.ok(contentsByModule.Ok[0] === content_addr.Ok)
  await s.consistency();

})

orchestrator.registerScenario("Scenario12: Delete content", async (s, t) => {
  const { alice, bob } = await s.players(
    {alice: conductorConfig, bob: conductorConfig},
    true
  );

  const course_addr = await create_course(alice, "my course", 123);
  t.ok(course_addr.Ok);
  await s.consistency();

  const module_addr = await create_module(alice, "my module", course_addr.Ok, 123);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr = await create_content(alice, "my content", module_addr.Ok, "www.content.com", 123, "this is my new content");
  t.ok(content_addr.Ok);
  await s.consistency();

  await alice.call("course_dna", "courses", "delete_content", {
    content_address: content_addr.Ok
  })
  await s.consistency();

  const contentsByModule = await alice.call("course_dna", "courses", "get_contents", {
    module_address: module_addr.Ok
  })
  t.ok(contentsByModule.Ok);
  t.ok(contentsByModule.Ok.length === 0)

})

orchestrator.run();