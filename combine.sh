#!/bin/sh

for i in `seq 1000 50 2600`
do
#    combineCards.py datacards/CMS_jj_HbbZqqHww_1000_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HbbZqqHww_1000_8TeV_CMS_jj_HwwLP.txt datacards/
    
    #Combine HbbZqq of HP and LP, Hbb tagger
    combineCards.py datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbLP.txt > datacardsCom/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHPLP.txt &

    #Combine HbbWqq of HP and LP, Hbb tagger
    combineCards.py datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt > datacardsCom/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHPLP.txt &

    #Combine HwwZqq of HP, LPV, LPH, Hww tagger
    combineCards.py datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsCom/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHPLPHV.txt &

    #Combine HwwWqq of HP, LPV, LPH, Hww tagger
    combineCards.py datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsCom/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHPLPHV.txt &

    #Combine HbbZqq of HP, LPV, LPH, Hww tagger
   # combineCards.py datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsCom/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwHPLPHV.txt &

    #Combine HbbWqq of HP, LPV, LPH, Hww tagger
    #combineCards.py datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsCom/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwHPLPHV.txt &

    #-----------------------------------------------------------------------------------
    #Combine all three channels together, HZ(qq)
    #combineCards.py datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPH.txt datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HbbZqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HZqq_$i\_8TeV_CMS_jj_HZCombined.txt &
    combineCards.py datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HZqq_$i\_8TeV_CMS_jj_HZCombined.txt &

 
   #combine all channels together, HW(qq)
   #combineCards.py datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HbbWqqHww_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HWqq_$i\_8TeV_CMS_jj_HWCombined.txt &
   combineCards.py datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HWqq_$i\_8TeV_CMS_jj_HWCombined.txt &

done
