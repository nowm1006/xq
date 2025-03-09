split("\n")|map(split(","))|
    map({"A":.[0]|tonumber,
         "B":.[1],
         "C":.[2],
    })