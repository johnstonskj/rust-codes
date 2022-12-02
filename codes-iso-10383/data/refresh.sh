curl  -o ./ISO10383_MIC-in.csv "https://www.iso20022.org/sites/default/files/ISO10383_MIC/ISO10383_MIC.csv"
iconv --byte-subst=Ñ -t UTF-8 ./ISO10383_MIC-in.csv > ./ISO10383_MIC.csv 
