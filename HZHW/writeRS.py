#####################
#####################
## write the background and signal rs file
###################################

signalN = [ 'HwwZqq', 'HwwWqq', 'HbbZqqHww', 'HbbWqqHww', 'HbbZqq', 'HbbWqq']
catego  = [ 'CMS_jj_HwwHP', 'CMS_jj_HwwLPV', 'CMS_jj_HwwLPH', 'CMS_jj_HbbHP', 'CMS_jj_HbbLP']

f = open( 'Xhv.rs', 'w' )
f.write( 'mgg[890,2500];\n')
for signal in signalN :
    f.write('\n')
    sigS = 'jj_'+signal + '_sig_'
    f.write( sigS + 'm0[2000.0, 900.0, 3100.0];' +  '\n'  )
    f.write( sigS + 'sigma[100, 50.0, 1000.0];'  +  '\n' )
    f.write( sigS + 'alpha[0.8, 0.5, 3];'  +  '\n' )
    f.write( sigS + 'n[13.0, 0.5, 10];'  +  '\n' )
    f.write( sigS + 'gsigma[100, 50.0, 1000.0];'  +  '\n' )
    f.write( sigS + 'frac[0.5, 0.01, 1.0];'  +  '\n' )
    f.write( '\n' )
    f.write( 'jjGaussSig' + signal + ' = ' + 'Gaussian(mgg, ' + sigS + 'm0, ' + sigS + 'gsigma);' + '\n'  )
    f.write( 'jjCBSig' + signal + ' = ' + 'CBShape(mgg, ' + sigS + 'm0, ' + sigS + 'sigma, ' + sigS + 'alpha, ' + sigS + 'n);' + '\n' )
    f.write( signal + '_jj' + ' = ' + 'AddPdf(' + 'jjGaussSig' + signal + ', jjCBSig' + signal + ', ' + sigS + 'frac);' + '\n'      )
    f.write( '\n' )
    for cat in catego :
	f.write( '\n' )
	f.write( sigS + 'm0_' + cat + '[2000.0, 900.0, 3100.0];' + '\n' )
	f.write( sigS + 'sigma_' + cat + '[100, 50.0, 1000.0];'  + '\n'  )
	f.write( sigS + 'alpha_' + cat + '[ 0.8, 0.0, 3.0];'  + '\n'  )
	f.write( sigS + 'n_' + cat + '[12, 0.00001, 10.0];'  + '\n'  )
	f.write( sigS + 'gsigma_' + cat + '[50, 20.0, 200.0];'  + '\n'  )
	f.write( sigS + 'frac_' + cat + '[0.5, 0.01, 1.0];'  + '\n'  )
	f.write( '\n' )
	f.write( 'jjGaussSig' + signal + '_' + cat + ' = ' + 'Gaussian(mgg, ' + sigS + 'm0_' + cat + ', ' + \
	 			sigS + 'gsigma_' + cat + ');' + '\n'  )
	f.write( 'jjCBSig' + signal + '_' + cat + ' = ' + 'CBShape(mgg, ' + sigS + 'm0_' + cat + ', ' + \
			        sigS + 'sigma_' + cat + ', ' + sigS + 'alpha_' + cat + ', '+ sigS + 'n_' + cat + ');' + '\n' )
	f.write( signal + '_jj_' + cat  + ' = ' + 'AddPdf(' + 'jjGaussSig' + signal + '_' + cat   + \
		 ', jjCBSig' + signal + '_' + cat + ', ' + sigS + 'frac_' + cat + ');' + '\n'      )
	f.write( '\n')


f.write( '\n' )
f.write( 'bkg_fit_slope[1.0,0, 1];\n' )
f.write( 'bkg_fit_slope1[7,0.0, 100.0];\n' )
f.write( 'bkg_fit_slope2[5,0.0, 100.0];\n' )
f.write( 'bkg_fit_slope3[0.,0.0, 0.0];\n' )

for cat in catego :
    f.write( '\n' )
    f.write( 'bkg_fit_slope_' + cat + '[1000.0,0, 10000000];'  + '\n' )
    f.write( 'bkg_fit_slope1_' + cat + '[10., -100.0, 500.0];' + '\n'  )
    f.write( 'bkg_fit_slope2_' + cat + '[5.,0.0, 100.0];'    + '\n'   )
    f.write( 'bkg_fit_slope3_' + cat + '[0.,-10.0, 10.0];'   + '\n'   )
    f.write( '\n' )

f.write( 'wei[1,0,10];' + '\n' )

f.write( 'sqrtS[8000., 8000., 8000.];' + '\n' )

f.close()

