# Rust and MongoDB based proposal for restcha test

This project is a pure academic aproach to a full productive rest-api application focused on fast and effective response that participates in the **restcha** test.

The **restcha** test is a high performance api-rest stress based challenge focused in productive environments. Tecnologies regarding the web server and/or web frameworks and ORMs are up to the developer, the only constraint is that the data must be stored in a **MongoDB** database.

> Curently the *restcha test* is in WIP, so this project will be updated when the alfa version of the test is released

Take into account that the technologies used in this project are meant to be a personal preference, and don't need to be the way to go cutting edge technologies. If you find the *restcha test* interesting you can contribute to any of the competing projects, launch your own competing project or even contribute to the test itself.

## Database installation

In order to run this applications tests a ``MongoDB`` (version 4 or later) named **store** is required with a collection named **pet**. The default database host is set to ``localhost`` and the default port to ``27017``, the values can be changed in the main module under MongoDB configration constants.