#!/bin/sh

#root -b -q MiniTreeProducerHZ.C
#root -b -q MiniTreeSignalProducerHV.C

#for i in 1000 1300 1500 1600 1800 2000 2200
for i in  2000
#for i in `seq 1000 50 2600`
do
    #all combined
    combine datacardsAll/CMS_jj_HWqq_$i\_8TeV_CMS_jj_HWCombined.txt -M Asymptotic -v2 -m $i -n HWqq --rMax 1000 --rMin 0.01 &
    combine datacardsAll/CMS_jj_HZqq_$i\_8TeV_CMS_jj_HZCombined.txt -M Asymptotic -v2 -m $i -n HZqq --rMax 1000 --rMin 0.01 &


done 
