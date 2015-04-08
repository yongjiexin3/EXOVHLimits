#!/bin/sh


#for i in  1000
for i in `seq 1000 50 2600`
do

    combine datacardsAll/CMS_jj_HVqq_$i\_8TeV_CMS_jj_HVCombined.txt -M Asymptotic -v2 -m $i -n HVqq --rMax 1000 --rMin 0.01 &
done 
