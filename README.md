# tanium-log-analyzer
Easily parse Tanium logs to find the information you need quickly.

## V1
Current functionality will analyze the latest Tanium Deploy log and display Self Service and Deploy Job IDs.
You can then enter the Job ID you want and see logs just for that job. It also filters our some noisy logs.

### Create Executable
1. Clone the repository
2. cd into the root of the project
3. Run ```cargo build --release ```
4. Get the executable from .\target\release
