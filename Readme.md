# Stellantis Connected Car API in Rust

With this tool you can aquire the car status of newer Stellantis Brand cars (Citroen, Opel, Vauxhall, DS, Peugeot).

## Initialize

You need the corresponding Brand Android APK for your car (MyCitroen, MyOpel, MyVauxhall, MyDS, MyPeugeot). The tool parses nessesary informations from the APK.

On first launch you have to input the path to the APK and your account credentials. The tool storage all config in the `config.yaml` file.

## Run

After the initialization the tool asks for the VIN of your car and debug print out the vehicle status response.
