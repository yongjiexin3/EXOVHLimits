import os

masses =[m*100/2 for m in range(2*10,2*29+1)]

for mass in masses:
  outputname = "submit_VV_"+str(mass)+".src"
  logname = "submit_VV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter.cc("+str(mass)+","+'""'+",0)'\n")
  outputfile.close
  
  command="rm "+logname
  print command
  os.system(command)
  command="bsub -q 1nh -o "+logname+" source "+outputname
  print command
  os.system(command)

masses =[m*100/2 for m in range(2*10,2*40+1)]

for mass in masses:
  outputname = "submit_qV_"+str(mass)+".src"
  logname = "submit_qV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter.cc("+str(mass)+","+'""'+",1)'\n")
  outputfile.close
  
  command="rm "+logname
  print command
  os.system(command)
  command="bsub -q 1nh -o "+logname+" source "+outputname
  print command
  os.system(command)

masses =[m*100/2 for m in range(2*10,2*29+1)]

for mass in masses:
  outputname = "submit_BulkVV_"+str(mass)+".src"
  logname = "submit_BulkVV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter.cc("+str(mass)+","+'""'+",2)'\n")
  outputfile.close
  
  command="rm "+logname
  print command
  os.system(command)
  command="bsub -q 1nh -o "+logname+" source "+outputname
  print command
  os.system(command)

