#!/bin/sh

#root -b -q MiniTreeProducerHZ.C
#root -b -q MiniTreeSignalProducerHV.C

#for i in 1000 1300 1500 1600 1800 2000 2200
#for i in  1000
for i in `seq 1000 50 2600`
do
    #combine datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt -M Asymptotic -v2 -m $i -n HbbWqq --rMax 1000 --rMin 0.01  
    #combine datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt -M Asymptotic -v2 -m $i -n HbbWqq --rMax 1000 --rMin 0.01  

    #HbbZqq  HP, LP
   # combine datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt -M Asymptotic -v2 -m $i -n HbbZqqHP --rMax 1000 --rMin 0.01 > log$i & 
   # combine datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbLP.txt -M Asymptotic -v2 -m $i -n HbbZqqLP --rMax 1000 --rMin 0.01  
    combine datacardsCom/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHPLP.txt -M Asymptotic -v2 -m $i -n HbbZqqHPLP --rMax 1000 --rMin 0.01 & 

    #HbbWqq  HP, LP
   # combine datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt -M Asymptotic -v2 -m $i -n HbbWqqHP --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt -M Asymptotic -v2 -m $i -n HbbWqqLP --rMax 1000 --rMin 0.01  
    combine datacardsCom/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHPLP.txt -M Asymptotic -v2 -m $i -n HbbWqqHPLP --rMax 1000 --rMin 0.01  &

     #HwwZqq  HP, LP
   # combine datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHP.txt -M Asymptotic -v2 -m $i -n HwwZqqHP --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPH.txt -M Asymptotic -v2 -m $i -n HwwZqqLPH --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPV.txt -M Asymptotic -v2 -m $i -n HwwZqqLPV --rMax 1000 --rMin 0.01  
    combine datacardsCom/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHPLPHV.txt -M Asymptotic -v2 -m $i -n HwwZqqHPLPHV --rMax 1000 --rMin 0.01 &  

    #HwwWqq  HP, LP
   # combine datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt -M Asymptotic -v2 -m $i -n HwwWqqHP --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt -M Asymptotic -v2 -m $i -n HwwWqqLPH --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt -M Asymptotic -v2 -m $i -n HwwWqqLPV --rMax 1000 --rMin 0.01  
    combine datacardsCom/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHPLPHV.txt -M Asymptotic -v2 -m $i -n HwwWqqHPLPHV --rMax 1000 --rMin 0.01  &

    #HbbWqq  HP, LP
   # combine datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwHP.txt -M Asymptotic -v2 -m $i -n HbbWqqHwwHP --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt -M Asymptotic -v2 -m $i -n HbbWqqHwwLPH --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt -M Asymptotic -v2 -m $i -n HbbWqqHwwLPV --rMax 1000 --rMin 0.01  
   # combine datacardsCom/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwHPLPHV.txt -M Asymptotic -v2 -m $i -n HbbWqqHwwHPLPHV --rMax 1000 --rMin 0.01  &

    #HbbZqq  HP, LP
   # combine datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwHP.txt -M Asymptotic -v2 -m $i -n HbbZqqHwwHP --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt -M Asymptotic -v2 -m $i -n HbbZqqHwwLPH --rMax 1000 --rMin 0.01  
   # combine datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt -M Asymptotic -v2 -m $i -n HbbZqqHwwLPV --rMax 1000 --rMin 0.01  
   # combine datacardsCom/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwHPLPHV.txt -M Asymptotic -v2 -m $i -n HbbZqqHwwHPLPHV --rMax 1000 --rMin 0.01  

    #------------------------------------------------------------------
    #all combined
   # combine datacardsAll/CMS_jj_HWqq_$i\_8TeV_CMS_jj_HWCombined.txt -M Asymptotic -v2 -m $i -n HWqq --rMax 1000 --rMin 0.01 &
   # combine datacardsAll/CMS_jj_HZqq_$i\_8TeV_CMS_jj_HZCombined.txt -M Asymptotic -v2 -m $i -n HZqq --rMax 1000 --rMin 0.01 &


done 
