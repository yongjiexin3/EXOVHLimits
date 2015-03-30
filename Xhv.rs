mgg[890,2500];

jj_HwwZqq_sig_m0[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha[0.8, 0.5, 3];
jj_HwwZqq_sig_n[13.0, 0.5, 10];
jj_HwwZqq_sig_gsigma[100, 50.0, 1000.0];
jj_HwwZqq_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHwwZqq = Gaussian(mgg, jj_HwwZqq_sig_m0, jj_HwwZqq_sig_gsigma);
jjCBSigHwwZqq = CBShape(mgg, jj_HwwZqq_sig_m0, jj_HwwZqq_sig_sigma, jj_HwwZqq_sig_alpha, jj_HwwZqq_sig_n);
HwwZqq_jj = AddPdf(jjGaussSigHwwZqq, jjCBSigHwwZqq, jj_HwwZqq_sig_frac);


jj_HwwZqq_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HwwZqq_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HwwZqq_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HwwZqq_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHwwZqq_CMS_jj_HwwHP = Gaussian(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwHP, jj_HwwZqq_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHwwZqq_CMS_jj_HwwHP = CBShape(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwHP, jj_HwwZqq_sig_sigma_CMS_jj_HwwHP, jj_HwwZqq_sig_alpha_CMS_jj_HwwHP, jj_HwwZqq_sig_n_CMS_jj_HwwHP);
HwwZqq_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHwwZqq_CMS_jj_HwwHP, jjCBSigHwwZqq_CMS_jj_HwwHP, jj_HwwZqq_sig_frac_CMS_jj_HwwHP);


jj_HwwZqq_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HwwZqq_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HwwZqq_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HwwZqq_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHwwZqq_CMS_jj_HwwLPV = Gaussian(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwLPV, jj_HwwZqq_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHwwZqq_CMS_jj_HwwLPV = CBShape(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwLPV, jj_HwwZqq_sig_sigma_CMS_jj_HwwLPV, jj_HwwZqq_sig_alpha_CMS_jj_HwwLPV, jj_HwwZqq_sig_n_CMS_jj_HwwLPV);
HwwZqq_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHwwZqq_CMS_jj_HwwLPV, jjCBSigHwwZqq_CMS_jj_HwwLPV, jj_HwwZqq_sig_frac_CMS_jj_HwwLPV);


jj_HwwZqq_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HwwZqq_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HwwZqq_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HwwZqq_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHwwZqq_CMS_jj_HwwLPH = Gaussian(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwLPH, jj_HwwZqq_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHwwZqq_CMS_jj_HwwLPH = CBShape(mgg, jj_HwwZqq_sig_m0_CMS_jj_HwwLPH, jj_HwwZqq_sig_sigma_CMS_jj_HwwLPH, jj_HwwZqq_sig_alpha_CMS_jj_HwwLPH, jj_HwwZqq_sig_n_CMS_jj_HwwLPH);
HwwZqq_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHwwZqq_CMS_jj_HwwLPH, jjCBSigHwwZqq_CMS_jj_HwwLPH, jj_HwwZqq_sig_frac_CMS_jj_HwwLPH);


jj_HwwZqq_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HwwZqq_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HwwZqq_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HwwZqq_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHwwZqq_CMS_jj_HbbHP = Gaussian(mgg, jj_HwwZqq_sig_m0_CMS_jj_HbbHP, jj_HwwZqq_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHwwZqq_CMS_jj_HbbHP = CBShape(mgg, jj_HwwZqq_sig_m0_CMS_jj_HbbHP, jj_HwwZqq_sig_sigma_CMS_jj_HbbHP, jj_HwwZqq_sig_alpha_CMS_jj_HbbHP, jj_HwwZqq_sig_n_CMS_jj_HbbHP);
HwwZqq_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHwwZqq_CMS_jj_HbbHP, jjCBSigHwwZqq_CMS_jj_HbbHP, jj_HwwZqq_sig_frac_CMS_jj_HbbHP);


jj_HwwZqq_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HwwZqq_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HwwZqq_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HwwZqq_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HwwZqq_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HwwZqq_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHwwZqq_CMS_jj_HbbLP = Gaussian(mgg, jj_HwwZqq_sig_m0_CMS_jj_HbbLP, jj_HwwZqq_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHwwZqq_CMS_jj_HbbLP = CBShape(mgg, jj_HwwZqq_sig_m0_CMS_jj_HbbLP, jj_HwwZqq_sig_sigma_CMS_jj_HbbLP, jj_HwwZqq_sig_alpha_CMS_jj_HbbLP, jj_HwwZqq_sig_n_CMS_jj_HbbLP);
HwwZqq_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHwwZqq_CMS_jj_HbbLP, jjCBSigHwwZqq_CMS_jj_HbbLP, jj_HwwZqq_sig_frac_CMS_jj_HbbLP);


jj_HwwWqq_sig_m0[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha[0.8, 0.5, 3];
jj_HwwWqq_sig_n[13.0, 0.5, 10];
jj_HwwWqq_sig_gsigma[100, 50.0, 1000.0];
jj_HwwWqq_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHwwWqq = Gaussian(mgg, jj_HwwWqq_sig_m0, jj_HwwWqq_sig_gsigma);
jjCBSigHwwWqq = CBShape(mgg, jj_HwwWqq_sig_m0, jj_HwwWqq_sig_sigma, jj_HwwWqq_sig_alpha, jj_HwwWqq_sig_n);
HwwWqq_jj = AddPdf(jjGaussSigHwwWqq, jjCBSigHwwWqq, jj_HwwWqq_sig_frac);


jj_HwwWqq_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HwwWqq_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HwwWqq_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HwwWqq_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHwwWqq_CMS_jj_HwwHP = Gaussian(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwHP, jj_HwwWqq_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHwwWqq_CMS_jj_HwwHP = CBShape(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwHP, jj_HwwWqq_sig_sigma_CMS_jj_HwwHP, jj_HwwWqq_sig_alpha_CMS_jj_HwwHP, jj_HwwWqq_sig_n_CMS_jj_HwwHP);
HwwWqq_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHwwWqq_CMS_jj_HwwHP, jjCBSigHwwWqq_CMS_jj_HwwHP, jj_HwwWqq_sig_frac_CMS_jj_HwwHP);


jj_HwwWqq_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HwwWqq_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HwwWqq_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HwwWqq_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHwwWqq_CMS_jj_HwwLPV = Gaussian(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwLPV, jj_HwwWqq_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHwwWqq_CMS_jj_HwwLPV = CBShape(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwLPV, jj_HwwWqq_sig_sigma_CMS_jj_HwwLPV, jj_HwwWqq_sig_alpha_CMS_jj_HwwLPV, jj_HwwWqq_sig_n_CMS_jj_HwwLPV);
HwwWqq_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHwwWqq_CMS_jj_HwwLPV, jjCBSigHwwWqq_CMS_jj_HwwLPV, jj_HwwWqq_sig_frac_CMS_jj_HwwLPV);


jj_HwwWqq_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HwwWqq_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HwwWqq_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HwwWqq_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHwwWqq_CMS_jj_HwwLPH = Gaussian(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwLPH, jj_HwwWqq_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHwwWqq_CMS_jj_HwwLPH = CBShape(mgg, jj_HwwWqq_sig_m0_CMS_jj_HwwLPH, jj_HwwWqq_sig_sigma_CMS_jj_HwwLPH, jj_HwwWqq_sig_alpha_CMS_jj_HwwLPH, jj_HwwWqq_sig_n_CMS_jj_HwwLPH);
HwwWqq_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHwwWqq_CMS_jj_HwwLPH, jjCBSigHwwWqq_CMS_jj_HwwLPH, jj_HwwWqq_sig_frac_CMS_jj_HwwLPH);


jj_HwwWqq_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HwwWqq_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HwwWqq_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HwwWqq_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHwwWqq_CMS_jj_HbbHP = Gaussian(mgg, jj_HwwWqq_sig_m0_CMS_jj_HbbHP, jj_HwwWqq_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHwwWqq_CMS_jj_HbbHP = CBShape(mgg, jj_HwwWqq_sig_m0_CMS_jj_HbbHP, jj_HwwWqq_sig_sigma_CMS_jj_HbbHP, jj_HwwWqq_sig_alpha_CMS_jj_HbbHP, jj_HwwWqq_sig_n_CMS_jj_HbbHP);
HwwWqq_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHwwWqq_CMS_jj_HbbHP, jjCBSigHwwWqq_CMS_jj_HbbHP, jj_HwwWqq_sig_frac_CMS_jj_HbbHP);


jj_HwwWqq_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HwwWqq_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HwwWqq_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HwwWqq_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HwwWqq_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HwwWqq_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHwwWqq_CMS_jj_HbbLP = Gaussian(mgg, jj_HwwWqq_sig_m0_CMS_jj_HbbLP, jj_HwwWqq_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHwwWqq_CMS_jj_HbbLP = CBShape(mgg, jj_HwwWqq_sig_m0_CMS_jj_HbbLP, jj_HwwWqq_sig_sigma_CMS_jj_HbbLP, jj_HwwWqq_sig_alpha_CMS_jj_HbbLP, jj_HwwWqq_sig_n_CMS_jj_HbbLP);
HwwWqq_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHwwWqq_CMS_jj_HbbLP, jjCBSigHwwWqq_CMS_jj_HbbLP, jj_HwwWqq_sig_frac_CMS_jj_HbbLP);


jj_HbbZqqHww_sig_m0[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha[0.8, 0.5, 3];
jj_HbbZqqHww_sig_n[13.0, 0.5, 10];
jj_HbbZqqHww_sig_gsigma[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww = Gaussian(mgg, jj_HbbZqqHww_sig_m0, jj_HbbZqqHww_sig_gsigma);
jjCBSigHbbZqqHww = CBShape(mgg, jj_HbbZqqHww_sig_m0, jj_HbbZqqHww_sig_sigma, jj_HbbZqqHww_sig_alpha, jj_HbbZqqHww_sig_n);
HbbZqqHww_jj = AddPdf(jjGaussSigHbbZqqHww, jjCBSigHbbZqqHww, jj_HbbZqqHww_sig_frac);


jj_HbbZqqHww_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HbbZqqHww_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HbbZqqHww_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww_CMS_jj_HwwHP = Gaussian(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwHP, jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHbbZqqHww_CMS_jj_HwwHP = CBShape(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwHP, jj_HbbZqqHww_sig_sigma_CMS_jj_HwwHP, jj_HbbZqqHww_sig_alpha_CMS_jj_HwwHP, jj_HbbZqqHww_sig_n_CMS_jj_HwwHP);
HbbZqqHww_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHbbZqqHww_CMS_jj_HwwHP, jjCBSigHbbZqqHww_CMS_jj_HwwHP, jj_HbbZqqHww_sig_frac_CMS_jj_HwwHP);


jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HbbZqqHww_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HbbZqqHww_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww_CMS_jj_HwwLPV = Gaussian(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPV, jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHbbZqqHww_CMS_jj_HwwLPV = CBShape(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPV, jj_HbbZqqHww_sig_sigma_CMS_jj_HwwLPV, jj_HbbZqqHww_sig_alpha_CMS_jj_HwwLPV, jj_HbbZqqHww_sig_n_CMS_jj_HwwLPV);
HbbZqqHww_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHbbZqqHww_CMS_jj_HwwLPV, jjCBSigHbbZqqHww_CMS_jj_HwwLPV, jj_HbbZqqHww_sig_frac_CMS_jj_HwwLPV);


jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HbbZqqHww_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HbbZqqHww_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww_CMS_jj_HwwLPH = Gaussian(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPH, jj_HbbZqqHww_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHbbZqqHww_CMS_jj_HwwLPH = CBShape(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HwwLPH, jj_HbbZqqHww_sig_sigma_CMS_jj_HwwLPH, jj_HbbZqqHww_sig_alpha_CMS_jj_HwwLPH, jj_HbbZqqHww_sig_n_CMS_jj_HwwLPH);
HbbZqqHww_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHbbZqqHww_CMS_jj_HwwLPH, jjCBSigHbbZqqHww_CMS_jj_HwwLPH, jj_HbbZqqHww_sig_frac_CMS_jj_HwwLPH);


jj_HbbZqqHww_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HbbZqqHww_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HbbZqqHww_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HbbZqqHww_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww_CMS_jj_HbbHP = Gaussian(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HbbHP, jj_HbbZqqHww_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHbbZqqHww_CMS_jj_HbbHP = CBShape(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HbbHP, jj_HbbZqqHww_sig_sigma_CMS_jj_HbbHP, jj_HbbZqqHww_sig_alpha_CMS_jj_HbbHP, jj_HbbZqqHww_sig_n_CMS_jj_HbbHP);
HbbZqqHww_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHbbZqqHww_CMS_jj_HbbHP, jjCBSigHbbZqqHww_CMS_jj_HbbHP, jj_HbbZqqHww_sig_frac_CMS_jj_HbbHP);


jj_HbbZqqHww_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HbbZqqHww_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HbbZqqHww_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HbbZqqHww_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HbbZqqHww_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HbbZqqHww_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHbbZqqHww_CMS_jj_HbbLP = Gaussian(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HbbLP, jj_HbbZqqHww_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHbbZqqHww_CMS_jj_HbbLP = CBShape(mgg, jj_HbbZqqHww_sig_m0_CMS_jj_HbbLP, jj_HbbZqqHww_sig_sigma_CMS_jj_HbbLP, jj_HbbZqqHww_sig_alpha_CMS_jj_HbbLP, jj_HbbZqqHww_sig_n_CMS_jj_HbbLP);
HbbZqqHww_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHbbZqqHww_CMS_jj_HbbLP, jjCBSigHbbZqqHww_CMS_jj_HbbLP, jj_HbbZqqHww_sig_frac_CMS_jj_HbbLP);


jj_HbbWqqHww_sig_m0[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha[0.8, 0.5, 3];
jj_HbbWqqHww_sig_n[13.0, 0.5, 10];
jj_HbbWqqHww_sig_gsigma[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww = Gaussian(mgg, jj_HbbWqqHww_sig_m0, jj_HbbWqqHww_sig_gsigma);
jjCBSigHbbWqqHww = CBShape(mgg, jj_HbbWqqHww_sig_m0, jj_HbbWqqHww_sig_sigma, jj_HbbWqqHww_sig_alpha, jj_HbbWqqHww_sig_n);
HbbWqqHww_jj = AddPdf(jjGaussSigHbbWqqHww, jjCBSigHbbWqqHww, jj_HbbWqqHww_sig_frac);


jj_HbbWqqHww_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HbbWqqHww_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HbbWqqHww_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww_CMS_jj_HwwHP = Gaussian(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwHP, jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHbbWqqHww_CMS_jj_HwwHP = CBShape(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwHP, jj_HbbWqqHww_sig_sigma_CMS_jj_HwwHP, jj_HbbWqqHww_sig_alpha_CMS_jj_HwwHP, jj_HbbWqqHww_sig_n_CMS_jj_HwwHP);
HbbWqqHww_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHbbWqqHww_CMS_jj_HwwHP, jjCBSigHbbWqqHww_CMS_jj_HwwHP, jj_HbbWqqHww_sig_frac_CMS_jj_HwwHP);


jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HbbWqqHww_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HbbWqqHww_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww_CMS_jj_HwwLPV = Gaussian(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPV, jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHbbWqqHww_CMS_jj_HwwLPV = CBShape(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPV, jj_HbbWqqHww_sig_sigma_CMS_jj_HwwLPV, jj_HbbWqqHww_sig_alpha_CMS_jj_HwwLPV, jj_HbbWqqHww_sig_n_CMS_jj_HwwLPV);
HbbWqqHww_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHbbWqqHww_CMS_jj_HwwLPV, jjCBSigHbbWqqHww_CMS_jj_HwwLPV, jj_HbbWqqHww_sig_frac_CMS_jj_HwwLPV);


jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HbbWqqHww_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HbbWqqHww_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww_CMS_jj_HwwLPH = Gaussian(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPH, jj_HbbWqqHww_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHbbWqqHww_CMS_jj_HwwLPH = CBShape(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HwwLPH, jj_HbbWqqHww_sig_sigma_CMS_jj_HwwLPH, jj_HbbWqqHww_sig_alpha_CMS_jj_HwwLPH, jj_HbbWqqHww_sig_n_CMS_jj_HwwLPH);
HbbWqqHww_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHbbWqqHww_CMS_jj_HwwLPH, jjCBSigHbbWqqHww_CMS_jj_HwwLPH, jj_HbbWqqHww_sig_frac_CMS_jj_HwwLPH);


jj_HbbWqqHww_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HbbWqqHww_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HbbWqqHww_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HbbWqqHww_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww_CMS_jj_HbbHP = Gaussian(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HbbHP, jj_HbbWqqHww_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHbbWqqHww_CMS_jj_HbbHP = CBShape(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HbbHP, jj_HbbWqqHww_sig_sigma_CMS_jj_HbbHP, jj_HbbWqqHww_sig_alpha_CMS_jj_HbbHP, jj_HbbWqqHww_sig_n_CMS_jj_HbbHP);
HbbWqqHww_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHbbWqqHww_CMS_jj_HbbHP, jjCBSigHbbWqqHww_CMS_jj_HbbHP, jj_HbbWqqHww_sig_frac_CMS_jj_HbbHP);


jj_HbbWqqHww_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HbbWqqHww_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HbbWqqHww_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HbbWqqHww_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HbbWqqHww_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HbbWqqHww_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHbbWqqHww_CMS_jj_HbbLP = Gaussian(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HbbLP, jj_HbbWqqHww_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHbbWqqHww_CMS_jj_HbbLP = CBShape(mgg, jj_HbbWqqHww_sig_m0_CMS_jj_HbbLP, jj_HbbWqqHww_sig_sigma_CMS_jj_HbbLP, jj_HbbWqqHww_sig_alpha_CMS_jj_HbbLP, jj_HbbWqqHww_sig_n_CMS_jj_HbbLP);
HbbWqqHww_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHbbWqqHww_CMS_jj_HbbLP, jjCBSigHbbWqqHww_CMS_jj_HbbLP, jj_HbbWqqHww_sig_frac_CMS_jj_HbbLP);


jj_HbbZqq_sig_m0[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha[0.8, 0.5, 3];
jj_HbbZqq_sig_n[13.0, 0.5, 10];
jj_HbbZqq_sig_gsigma[100, 50.0, 1000.0];
jj_HbbZqq_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHbbZqq = Gaussian(mgg, jj_HbbZqq_sig_m0, jj_HbbZqq_sig_gsigma);
jjCBSigHbbZqq = CBShape(mgg, jj_HbbZqq_sig_m0, jj_HbbZqq_sig_sigma, jj_HbbZqq_sig_alpha, jj_HbbZqq_sig_n);
HbbZqq_jj = AddPdf(jjGaussSigHbbZqq, jjCBSigHbbZqq, jj_HbbZqq_sig_frac);


jj_HbbZqq_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HbbZqq_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HbbZqq_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HbbZqq_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHbbZqq_CMS_jj_HwwHP = Gaussian(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwHP, jj_HbbZqq_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHbbZqq_CMS_jj_HwwHP = CBShape(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwHP, jj_HbbZqq_sig_sigma_CMS_jj_HwwHP, jj_HbbZqq_sig_alpha_CMS_jj_HwwHP, jj_HbbZqq_sig_n_CMS_jj_HwwHP);
HbbZqq_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHbbZqq_CMS_jj_HwwHP, jjCBSigHbbZqq_CMS_jj_HwwHP, jj_HbbZqq_sig_frac_CMS_jj_HwwHP);


jj_HbbZqq_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HbbZqq_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HbbZqq_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HbbZqq_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHbbZqq_CMS_jj_HwwLPV = Gaussian(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwLPV, jj_HbbZqq_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHbbZqq_CMS_jj_HwwLPV = CBShape(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwLPV, jj_HbbZqq_sig_sigma_CMS_jj_HwwLPV, jj_HbbZqq_sig_alpha_CMS_jj_HwwLPV, jj_HbbZqq_sig_n_CMS_jj_HwwLPV);
HbbZqq_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHbbZqq_CMS_jj_HwwLPV, jjCBSigHbbZqq_CMS_jj_HwwLPV, jj_HbbZqq_sig_frac_CMS_jj_HwwLPV);


jj_HbbZqq_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HbbZqq_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HbbZqq_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HbbZqq_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHbbZqq_CMS_jj_HwwLPH = Gaussian(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwLPH, jj_HbbZqq_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHbbZqq_CMS_jj_HwwLPH = CBShape(mgg, jj_HbbZqq_sig_m0_CMS_jj_HwwLPH, jj_HbbZqq_sig_sigma_CMS_jj_HwwLPH, jj_HbbZqq_sig_alpha_CMS_jj_HwwLPH, jj_HbbZqq_sig_n_CMS_jj_HwwLPH);
HbbZqq_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHbbZqq_CMS_jj_HwwLPH, jjCBSigHbbZqq_CMS_jj_HwwLPH, jj_HbbZqq_sig_frac_CMS_jj_HwwLPH);


jj_HbbZqq_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HbbZqq_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HbbZqq_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HbbZqq_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHbbZqq_CMS_jj_HbbHP = Gaussian(mgg, jj_HbbZqq_sig_m0_CMS_jj_HbbHP, jj_HbbZqq_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHbbZqq_CMS_jj_HbbHP = CBShape(mgg, jj_HbbZqq_sig_m0_CMS_jj_HbbHP, jj_HbbZqq_sig_sigma_CMS_jj_HbbHP, jj_HbbZqq_sig_alpha_CMS_jj_HbbHP, jj_HbbZqq_sig_n_CMS_jj_HbbHP);
HbbZqq_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHbbZqq_CMS_jj_HbbHP, jjCBSigHbbZqq_CMS_jj_HbbHP, jj_HbbZqq_sig_frac_CMS_jj_HbbHP);


jj_HbbZqq_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HbbZqq_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HbbZqq_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HbbZqq_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HbbZqq_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HbbZqq_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHbbZqq_CMS_jj_HbbLP = Gaussian(mgg, jj_HbbZqq_sig_m0_CMS_jj_HbbLP, jj_HbbZqq_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHbbZqq_CMS_jj_HbbLP = CBShape(mgg, jj_HbbZqq_sig_m0_CMS_jj_HbbLP, jj_HbbZqq_sig_sigma_CMS_jj_HbbLP, jj_HbbZqq_sig_alpha_CMS_jj_HbbLP, jj_HbbZqq_sig_n_CMS_jj_HbbLP);
HbbZqq_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHbbZqq_CMS_jj_HbbLP, jjCBSigHbbZqq_CMS_jj_HbbLP, jj_HbbZqq_sig_frac_CMS_jj_HbbLP);


jj_HbbWqq_sig_m0[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha[0.8, 0.5, 3];
jj_HbbWqq_sig_n[13.0, 0.5, 10];
jj_HbbWqq_sig_gsigma[100, 50.0, 1000.0];
jj_HbbWqq_sig_frac[0.5, 0.01, 1.0];

jjGaussSigHbbWqq = Gaussian(mgg, jj_HbbWqq_sig_m0, jj_HbbWqq_sig_gsigma);
jjCBSigHbbWqq = CBShape(mgg, jj_HbbWqq_sig_m0, jj_HbbWqq_sig_sigma, jj_HbbWqq_sig_alpha, jj_HbbWqq_sig_n);
HbbWqq_jj = AddPdf(jjGaussSigHbbWqq, jjCBSigHbbWqq, jj_HbbWqq_sig_frac);


jj_HbbWqq_sig_m0_CMS_jj_HwwHP[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma_CMS_jj_HwwHP[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha_CMS_jj_HwwHP[ 0.8, 0.0, 3.0];
jj_HbbWqq_sig_n_CMS_jj_HwwHP[12, 0.00001, 10.0];
jj_HbbWqq_sig_gsigma_CMS_jj_HwwHP[50, 20.0, 200.0];
jj_HbbWqq_sig_frac_CMS_jj_HwwHP[0.5, 0.01, 1.0];

jjGaussSigHbbWqq_CMS_jj_HwwHP = Gaussian(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwHP, jj_HbbWqq_sig_gsigma_CMS_jj_HwwHP);
jjCBSigHbbWqq_CMS_jj_HwwHP = CBShape(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwHP, jj_HbbWqq_sig_sigma_CMS_jj_HwwHP, jj_HbbWqq_sig_alpha_CMS_jj_HwwHP, jj_HbbWqq_sig_n_CMS_jj_HwwHP);
HbbWqq_jj_CMS_jj_HwwHP = AddPdf(jjGaussSigHbbWqq_CMS_jj_HwwHP, jjCBSigHbbWqq_CMS_jj_HwwHP, jj_HbbWqq_sig_frac_CMS_jj_HwwHP);


jj_HbbWqq_sig_m0_CMS_jj_HwwLPV[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma_CMS_jj_HwwLPV[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha_CMS_jj_HwwLPV[ 0.8, 0.0, 3.0];
jj_HbbWqq_sig_n_CMS_jj_HwwLPV[12, 0.00001, 10.0];
jj_HbbWqq_sig_gsigma_CMS_jj_HwwLPV[50, 20.0, 200.0];
jj_HbbWqq_sig_frac_CMS_jj_HwwLPV[0.5, 0.01, 1.0];

jjGaussSigHbbWqq_CMS_jj_HwwLPV = Gaussian(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwLPV, jj_HbbWqq_sig_gsigma_CMS_jj_HwwLPV);
jjCBSigHbbWqq_CMS_jj_HwwLPV = CBShape(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwLPV, jj_HbbWqq_sig_sigma_CMS_jj_HwwLPV, jj_HbbWqq_sig_alpha_CMS_jj_HwwLPV, jj_HbbWqq_sig_n_CMS_jj_HwwLPV);
HbbWqq_jj_CMS_jj_HwwLPV = AddPdf(jjGaussSigHbbWqq_CMS_jj_HwwLPV, jjCBSigHbbWqq_CMS_jj_HwwLPV, jj_HbbWqq_sig_frac_CMS_jj_HwwLPV);


jj_HbbWqq_sig_m0_CMS_jj_HwwLPH[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma_CMS_jj_HwwLPH[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha_CMS_jj_HwwLPH[ 0.8, 0.0, 3.0];
jj_HbbWqq_sig_n_CMS_jj_HwwLPH[12, 0.00001, 10.0];
jj_HbbWqq_sig_gsigma_CMS_jj_HwwLPH[50, 20.0, 200.0];
jj_HbbWqq_sig_frac_CMS_jj_HwwLPH[0.5, 0.01, 1.0];

jjGaussSigHbbWqq_CMS_jj_HwwLPH = Gaussian(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwLPH, jj_HbbWqq_sig_gsigma_CMS_jj_HwwLPH);
jjCBSigHbbWqq_CMS_jj_HwwLPH = CBShape(mgg, jj_HbbWqq_sig_m0_CMS_jj_HwwLPH, jj_HbbWqq_sig_sigma_CMS_jj_HwwLPH, jj_HbbWqq_sig_alpha_CMS_jj_HwwLPH, jj_HbbWqq_sig_n_CMS_jj_HwwLPH);
HbbWqq_jj_CMS_jj_HwwLPH = AddPdf(jjGaussSigHbbWqq_CMS_jj_HwwLPH, jjCBSigHbbWqq_CMS_jj_HwwLPH, jj_HbbWqq_sig_frac_CMS_jj_HwwLPH);


jj_HbbWqq_sig_m0_CMS_jj_HbbHP[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma_CMS_jj_HbbHP[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha_CMS_jj_HbbHP[ 0.8, 0.0, 3.0];
jj_HbbWqq_sig_n_CMS_jj_HbbHP[12, 0.00001, 10.0];
jj_HbbWqq_sig_gsigma_CMS_jj_HbbHP[50, 20.0, 200.0];
jj_HbbWqq_sig_frac_CMS_jj_HbbHP[0.5, 0.01, 1.0];

jjGaussSigHbbWqq_CMS_jj_HbbHP = Gaussian(mgg, jj_HbbWqq_sig_m0_CMS_jj_HbbHP, jj_HbbWqq_sig_gsigma_CMS_jj_HbbHP);
jjCBSigHbbWqq_CMS_jj_HbbHP = CBShape(mgg, jj_HbbWqq_sig_m0_CMS_jj_HbbHP, jj_HbbWqq_sig_sigma_CMS_jj_HbbHP, jj_HbbWqq_sig_alpha_CMS_jj_HbbHP, jj_HbbWqq_sig_n_CMS_jj_HbbHP);
HbbWqq_jj_CMS_jj_HbbHP = AddPdf(jjGaussSigHbbWqq_CMS_jj_HbbHP, jjCBSigHbbWqq_CMS_jj_HbbHP, jj_HbbWqq_sig_frac_CMS_jj_HbbHP);


jj_HbbWqq_sig_m0_CMS_jj_HbbLP[2000.0, 900.0, 3100.0];
jj_HbbWqq_sig_sigma_CMS_jj_HbbLP[100, 50.0, 1000.0];
jj_HbbWqq_sig_alpha_CMS_jj_HbbLP[ 0.8, 0.0, 3.0];
jj_HbbWqq_sig_n_CMS_jj_HbbLP[12, 0.00001, 10.0];
jj_HbbWqq_sig_gsigma_CMS_jj_HbbLP[50, 20.0, 200.0];
jj_HbbWqq_sig_frac_CMS_jj_HbbLP[0.5, 0.01, 1.0];

jjGaussSigHbbWqq_CMS_jj_HbbLP = Gaussian(mgg, jj_HbbWqq_sig_m0_CMS_jj_HbbLP, jj_HbbWqq_sig_gsigma_CMS_jj_HbbLP);
jjCBSigHbbWqq_CMS_jj_HbbLP = CBShape(mgg, jj_HbbWqq_sig_m0_CMS_jj_HbbLP, jj_HbbWqq_sig_sigma_CMS_jj_HbbLP, jj_HbbWqq_sig_alpha_CMS_jj_HbbLP, jj_HbbWqq_sig_n_CMS_jj_HbbLP);
HbbWqq_jj_CMS_jj_HbbLP = AddPdf(jjGaussSigHbbWqq_CMS_jj_HbbLP, jjCBSigHbbWqq_CMS_jj_HbbLP, jj_HbbWqq_sig_frac_CMS_jj_HbbLP);


bkg_fit_slope[1.0,0, 1];
bkg_fit_slope1[7,0.0, 100.0];
bkg_fit_slope2[5,0.0, 100.0];
bkg_fit_slope3[0.,0.0, 0.0];

bkg_fit_slope_CMS_jj_HwwHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_HwwHP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_HwwHP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_HwwHP[0.,-10.0, 10.0];


bkg_fit_slope_CMS_jj_HwwLPV[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_HwwLPV[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_HwwLPV[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_HwwLPV[0.,-10.0, 10.0];


bkg_fit_slope_CMS_jj_HwwLPH[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_HwwLPH[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_HwwLPH[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_HwwLPH[0.,-10.0, 10.0];


bkg_fit_slope_CMS_jj_HbbHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_HbbHP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_HbbHP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_HbbHP[0.,-10.0, 10.0];


bkg_fit_slope_CMS_jj_HbbLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_HbbLP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_HbbLP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_HbbLP[0.,-10.0, 10.0];

wei[1,0,10];
sqrtS[8000., 8000., 8000.];
