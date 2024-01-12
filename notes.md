#### Tanium Directory - Windows
C:\Program Files (x86)\Tanium\Tanium Client

#### Base Log location:
C:\Program Files (x86)\Tanium\Tanium Client\Logs
 - action-history0.txt
 - cs-tool-history0.txt (Use this to see if the extra tooling was installed)
 - enforce.txt (This doesn't look worth parsing)
 - sensor-history0.txt (Doesn't look like it would be that useful)


#### Logs for specific tooling:
##### Deploy (Software Management) Logs
C:\Program Files (x86)\Tanium\Tanium Client\Tools\SoftwareManagement\logs
 - software-management.log
 - subprocess.log 

 - Location of Deployment package files
C:\Program Files (x86)\Tanium\Tanium Client\Tools\SoftwareManagement\data\temp\tasks

###### Self Service
 - Should contain "EUSS Deploy #" (# being the deploy job)
 - We could just look for "EUSS Deploy" and then follow the below rules to remove clutter
 - Should ignore lines with "memoizing", "enumerating", "Evaluating", "Building EUSS"

 - Look for "Running step"
 - Look for "Step # complete."
 - Look for "Task completed"

 - Could have a function that lists all the Self Service Job IDs and name of software
 - Ex: "[EUSS Deploy 65 (FileZilla Client (x64 en-us) 3.60.2)]"
 - Could also go off of Software Package ID, which could be looked up in Tanium
 - Ex: "[Software package 2459 (Tim Kosse FileZilla Client (x64 en-US) 3.60.2)]"

