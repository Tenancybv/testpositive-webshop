
# SOAP Web Service Testing Project

[![Branch Coverage Status](./coverage-badge-branch.svg)](https://github.com/Tenancybv/testpositive-webshop/WebShopUnitTests/OpenCover/coverage.opencover.xml)

This project focuses on testing a SOAP web service using various testing methods:

- Unit testing
- GUI testing
- Performance testing
- API testing

The target SOAP web service is available at:

http://demowebshop.tricentis.com/Plugins/Misc.WebServicesCustomer/Remote/NopService.svc?wsdl

## Getting Started

1. Clone this repository.
2. Set up the required environment and tools based on the testing suites you choose to use.
3. Uncomment the appropriate sections in the `.github/workflows/ci_pipeline.yml` file to include the testing tools and commands.
4. Push your changes to the main branch or create a pull request to trigger the GitHub Actions CI pipeline.

## CI Pipeline

The project includes a GitHub Actions CI pipeline that runs automatically on every push and pull request to the main branch. The pipeline is defined in the `.github/workflows/ci_pipeline.yml` file.

Currently, the pipeline is set up to work with a C# project. The specific testing tools and commands are commented out and should be replaced with the testing suites you decide to use for each testing method.

## Contributing

To contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your changes.
3. Commit your changes and push them to your fork.
4. Create a pull request to merge your changes into the main branch.

## License

This project is open source and available under the [MIT License](LICENSE).