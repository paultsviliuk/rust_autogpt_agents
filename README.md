# rust_autogpt_agents

Project manager

This agent will be the one calling the shots and directing other agents to do the required work. It’ll start by creating the required agents for the project. This agent will have access to a fact sheet that contains all of the requirements that the project must fulfill in order to be considered complete. The project manager gives the solutions architect agent a description of the project that the user wants them to create.

Solutions architect

The solutions architect will define the scope of the project and will assign tasks to the backend developer. For example, the solutions architect will use the project description to determine whether any external APIs are required for the application and, based on that information, generate URLs for APIs that the backend developer can use.

Backend developer

The backend developer will use the data provided by both the project manager and the solutions architect to write the code required for the application. It should also be noted that the backend developer uses an existing code template as a reference for the code it writes. After it is done writing the code, it will also perform unit testing to confirm if the code is safe to run and performing as expected.