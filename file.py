
HbbWqqRe = []
HbbZqqRe = []

HwwWqqRe = []
HwwZqqRe = []
for i in range(16) :
    a = 1000 + 100*i
    f = open( "logA_" + str(a) )
    lines = f.readlines()

    print "resonance mass  ", a
    counter = 0
    for line in lines :
	if 'jj_HbbWqq_sig_m0_CMS_jj_HwwHP' in line and '4' in line :
	    print line
	    counter = counter + 1
	    print counter
	    if counter == 4 :
	        HbbWqqRe.append( line.split()[2] )
    counter = 0
    for line in lines :
        if 'jj_HbbZqq_sig_m0_CMS_jj_HwwHP' in line and '4' in line :
            print line
            counter = counter + 1
            print counter
            if counter == 4 :
                HbbZqqRe.append( line.split()[2] )
    f.close()

    f = open( "logB_" + str(a) )
    lines = f.readlines()

    print "resonance mass  ", a
    counter = 0
    for line in lines :
        if 'jj_HwwWqq_sig_m0_CMS_jj_HwwHP' in line and '4' in line :
            print line
            counter = counter + 1
            print counter
            if counter == 4 :
                HwwWqqRe.append( line.split()[2] )
    counter = 0
    for line in lines :
        if 'jj_HwwZqq_sig_m0_CMS_jj_HwwHP' in line and '4' in line :
            print line
            counter = counter + 1
            print counter
            if counter == 4 :
                HwwZqqRe.append( line.split()[2] )
    f.close()


print HbbZqqRe
print HbbWqqRe
print HwwZqqRe
print HwwWqqRe

print "HbbZqq", "HbbWqq", "HwwZqq", "HwwWqq"
for i in range( 16 ) :
    #print HbbZqqRe[i], HbbWqqRe[i], HwwZqqRe[i], HwwWqqRe[i]
    print (float(HbbWqqRe[i])-float(HbbZqqRe[i]))/float(HbbZqqRe[i]), \
    (float(HwwZqqRe[i])-float(HbbWqqRe[i]))/float(HbbWqqRe[i]), (float(HwwWqqRe[i])-float(HwwZqqRe[i]))/float(HwwZqqRe[i]) 
#f2 = open( "ReSonanceMass.txt", "w" )
 
#f2.write( HbbWqqRe )

 

