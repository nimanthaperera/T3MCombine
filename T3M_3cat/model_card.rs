m3m[1.62,2.0];

sig_m0_A1[1.777, 1.72, 1.84];
sig_sigma_A1[0.02, 0.0, 0.1];
sig_alpha_A1[ 1, -5., 5.]; 
sig_n_A1[15, 0.0, 30.0]; 
sig_gaus_sigma_A1[0.01,0.0001,0.1];
cb_fraction_A1[0.5,0,1];

t3m_sig_CBshape_A1  = CBShape(m3m, sig_m0_A1, sig_sigma_A1, sig_alpha_A1, sig_n_A1);
t3m_sig_GSshape_A1  = Gaussian(m3m,sig_m0_A1,sig_gaus_sigma_A1);


sig_m0_B1[1.777, 1.72, 1.84];
sig_sigma_B1[0.02, 0.0, 0.05];
sig_alpha_B1[ 1, -20., 20.]; 
sig_n_B1[2, 0.0, 5.0]; 
sig_gaus_sigma_B1[0.02,0.0,0.1];
cb_fraction_B1[0.5,0,1];

t3m_sig_CBshape_B1  = CBShape(m3m, sig_m0_B1, sig_sigma_B1, sig_alpha_B1, sig_n_B1);
t3m_sig_GSshape_B1  = Gaussian(m3m,sig_m0_B1,sig_gaus_sigma_B1);


sig_m0_C1[1.777, 1.72, 1.84];
sig_sigma_C1[0.02, 0.0, 0.05];
sig_alpha_C1[ 1, -20., 20.]; 
sig_n_C1[2, 0.0, 5.0]; 
sig_gaus_sigma_C1[0.02,0.0,0.1];
cb_fraction_C1[0.5,0,1];

t3m_sig_CBshape_C1  = CBShape(m3m, sig_m0_C1, sig_sigma_C1, sig_alpha_C1, sig_n_C1);
t3m_sig_GSshape_C1  = Gaussian(m3m,sig_m0_C1,sig_gaus_sigma_C1);


sig_m0_A2[1.777, 1.72, 1.84];
sig_sigma_A2[0.02, 0.0, 0.05];
sig_alpha_A2[ 1, -20., 20.]; 
sig_n_A2[2, 0.0, 5.0]; 
sig_gaus_sigma_A2[0.02,0.0,0.1];
cb_fraction_A2[0.5,0,1];

t3m_sig_CBshape_A2  = CBShape(m3m, sig_m0_A2, sig_sigma_A2, sig_alpha_A2, sig_n_A2);
t3m_sig_GSshape_A2  = Gaussian(m3m,sig_m0_A2,sig_gaus_sigma_A2);


sig_m0_B2[1.777, 1.72, 1.84];
sig_sigma_B2[0.02, 0.0, 0.05];
sig_alpha_B2[ 1, -20., 20.]; 
sig_n_B2[2, 0.0, 5.0]; 
sig_gaus_sigma_B2[0.02,0.0,0.1];
cb_fraction_B2[0.5,0,1];

t3m_sig_CBshape_B2  = CBShape(m3m, sig_m0_B2, sig_sigma_B2, sig_alpha_B2, sig_n_B2);
t3m_sig_GSshape_B2  = Gaussian(m3m,sig_m0_B2,sig_gaus_sigma_B2);


sig_m0_C2[1.777, 1.72, 1.84];
sig_sigma_C2[0.02, 0.0, 0.05];
sig_alpha_C2[ 1, -20., 20.]; 
sig_n_C2[2, 0.0, 5.0]; 
sig_gaus_sigma_C2[0.02,0.0,0.1];
cb_fraction_C2[0.5,0,1];

t3m_sig_CBshape_C2  = CBShape(m3m, sig_m0_C2, sig_sigma_C2, sig_alpha_C2, sig_n_C2);
t3m_sig_GSshape_C2  = Gaussian(m3m,sig_m0_C2,sig_gaus_sigma_C2);


sig_m0_A3[1.777, 1.72, 1.84];
sig_sigma_A3[0.02, 0.0, 0.05];
sig_alpha_A3[ 1, -20., 20.]; 
sig_n_A3[2, 0.0, 5.0]; 
sig_gaus_sigma_A3[0.02,0.0,0.1];
cb_fraction_A3[0.5,0,1];

t3m_sig_CBshape_A3  = CBShape(m3m, sig_m0_A3, sig_sigma_A3, sig_alpha_A3, sig_n_A3);
t3m_sig_GSshape_A3  = Gaussian(m3m,sig_m0_A3,sig_gaus_sigma_A3);


sig_m0_B3[1.777, 1.72, 1.84];
sig_sigma_B3[0.02, 0.0, 0.05];
sig_alpha_B3[ 1, -20., 20.]; 
sig_n_B3[2, 0.0, 5.0]; 
sig_gaus_sigma_B3[0.02,0.0,0.1];
cb_fraction_B3[0.5,0,1];

t3m_sig_CBshape_B3  = CBShape(m3m, sig_m0_B3, sig_sigma_B3, sig_alpha_B3, sig_n_B3);
t3m_sig_GSshape_B3  = Gaussian(m3m,sig_m0_B3,sig_gaus_sigma_B3);


sig_m0_C3[1.777, 1.72, 1.84];
sig_sigma_C3[0.02, 0.0, 0.05];
sig_alpha_C3[ 1, -20., 20.]; 
sig_n_C3[2, 0.0, 5.0]; 
sig_gaus_sigma_C3[0.02,0.0,0.1];
cb_fraction_C3[0.5,0,1];

t3m_sig_CBshape_C3  = CBShape(m3m, sig_m0_C3, sig_sigma_C3, sig_alpha_C3, sig_n_C3);
t3m_sig_GSshape_C3  = Gaussian(m3m,sig_m0_C3,sig_gaus_sigma_C3);


bkg_exp_slope_A1[-3.0,-6.0,-0.0];
bkg_exp_slope_B1[-3.0,-6.0,-0.0];
bkg_exp_slope_C1[-3.0,-6.0,-0.0];
//bkg_exp_slope_C1[-5.3,-5.4,-5.2];

bkg_exp_slope_A2[-3.0,-6.0,-0.0];
bkg_exp_slope_B2[-3.0,-6.0,-0.0];
bkg_exp_slope_C2[-3.0,-6.0,-0.1];

bkg_exp_slope_A3[-3.0,-6.0,-0.0];
bkg_exp_slope_B3[-3.0,-6.0,-0.0];
bkg_exp_slope_C3[-3.0,-6.0,-0.1];

bkg_exp_offset_A1[0.0,-20.0,10.0];
bkg_exp_offset_B1[0.0,-20.0,10.0];
bkg_exp_offset_C1[0.0,-20.0,10.0];

bkg_exp_offset_A2[0.0,-20.0,10.0];
bkg_exp_offset_B2[0.0,-20.0,10.0];
bkg_exp_offset_C2[0.0,-10.0,10.0];

bkg_exp_offset_A3[0.0,-20.0,10.0];
bkg_exp_offset_B3[0.0,-20.0,10.0];
bkg_exp_offset_C3[0.0,-10.0,10.0];


//bkg_exp_shape_A1 = RooExponential(m3m,bkg_exp_slope_A1, bkg_exp_offset_A1);
//bkg_exp_shape_A2 = RooExponential(m3m,bkg_exp_slope_A2, bkg_exp_offset_A2);
//bkg_exp_shape_B1 = RooExponential(m3m,bkg_exp_slope_B1, bkg_exp_offset_B1);
//bkg_exp_shape_B2 = RooExponential(m3m,bkg_exp_slope_B2, bkg_exp_offset_B2);
//bkg_exp_shape_C1 = RooExponential(m3m,bkg_exp_slope_C1, bkg_exp_offset_C1);
//bkg_exp_shape_C2 = RooExponential(m3m,bkg_exp_slope_C2, bkg_exp_offset_C2);


//bkg_exp_slope_A1[-2.0,-6.0,-0.001];
//bkg_exp_slope_A2[-2.0,-6.0,-0.001];
//bkg_exp_slope_B1[-3.0,-6.0,-0.001];
//bkg_exp_slope_B2[-2.0,-6.0,-0.001];
//bkg_exp_slope_C1[-2.0,-6.0,-0.001];
//bkg_exp_slope_C2[-2.0,-6.0,-0.001];
//
//bkg_exp_offset_A1[0.0,0.0,5.0];
//bkg_exp_offset_A2[0.0,0.0,5.0];
//bkg_exp_offset_B1[0.0,0.0,5.0];
//bkg_exp_offset_B2[0.0,0.0,5.0];
//bkg_exp_offset_C1[0.0,0.0,5.0];
//bkg_exp_offset_C2[0.0,0.0,5.0];

sqrtS[13000., 13000., 13000.]

//bkg_exp_offset[0.0,-10.0,10.0];
//bkg_exp_shape  = RooExponential(m3m,bkg_exp_slope);