#!/bin/bash

# Function to handle company file download
download_company_file() {
    echo "Do you want to download the 'companies' file? (y/n)"
    read choice
    if [ "$choice" = "y" ]; then
        current_date=$(date +'%Y-%m-01')
        download_link="https://download.companieshouse.gov.uk/BasicCompanyDataAsOneFile-$current_date.zip"
#        rm -rf ../data/company.zip
#        rm -rf ../data/company_old.csv
#        rm -rf ../data/company.csv
#        curl $download_link --output ../data/company.zip
#        unzip ../data/company.zip -d ../data/
#        mv ../data/BasicCompanyDataAsOneFile-*.csv ../data/company_old.csv
        ./rename_csv_fields
        mongoimport --db company --collection company --type csv --headerline --file ../data/company.csv
    fi
}

# Function to handle persons with significant control file download
download_pws_file() {
    echo "Do you want to download the 'persons-with-significant-control-snapshot' file? (y/n)"
    read choice
    if [ "$choice" = "y" ]; then
        current_date=$(date +'%Y-%m-01')
        download_link="https://download.companieshouse.gov.uk/persons-with-significant-control-snapshot-$current_date.zip"
        curl $download_link --output ../data/pws.zip 
        rm -rf ../data/pws.csv
        unzip ../data/pws.zip -d ../data/
        mv ../data/persons-with-significant-control-snapshot-*.txt ../data/pws.json
        mongoimport --db company --collection person_with_sig --file ../data/pws.json
    fi
}

# Function to handle postcodes file download
download_postcodes_file() {
    echo "Do you want to download the 'postcodes' file? (y/n)"
    read choice
    if [ "$choice" = "y" ]; then
        curl https://www.doogal.co.uk/files/postcodes.zip --output ../data/postcodes.zip
        rm -rf ../data/postcodes.csv
        unzip ../data/postcodes.zip -d ../data/
        mongoimport --db company --collection postcodes --type csv --headerline --file ../data/postcodes.csv
    fi
}

# Call the functions
download_company_file
download_pws_file
download_postcodes_file