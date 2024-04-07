extern crate rand;
extern crate rand_distr;

use std::thread;
use std::time::Duration;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use std::sync::{Arc, RwLock};
use std::sync::{mpsc, Mutex};

fn main() {

    //MISC FUNCTIONS
    fn generate_norm_rand() -> f64 {
        //generates a random number from a normal distribution
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let z = normal.sample(&mut rng);
        z
    }

    fn generate_rand_rsi() -> f64 {
        // Simulated data for average gain and average loss
            let average_gain = rand::thread_rng().gen_range(0.0..10.0);
            let average_loss = rand::thread_rng().gen_range(0.0..10.0);
        
            // Calculate RS
            let rs = average_gain / average_loss;
        
            // Calculate RSI
            let rsi = 100.0 - (100.0 / (1.0 + rs));
        
            rsi
        }

    // STOCKS
    // Columns = Symbol,Price,Change after 1 day,Volume of traded shares,Sector,sell rate,buy rate,RSI
    let mut stocks = vec![
    ["AAPL".to_string(), "130.71".to_string(), "-3.98".to_string(), "3783".to_string(), "Technology".to_string(), "134.69".to_string(), "131.75106760630413".to_string(), "69.59491933781679".to_string()],
    ["MSFT".to_string(), "136.41".to_string(), "0.43".to_string(), "5953".to_string(), "Technology".to_string(), "136.95786896526383".to_string(), "136.84".to_string(), "46.39090849371409".to_string()],
    ["GOOGL".to_string(), "152.78".to_string(), "0.64".to_string(), "5989".to_string(), "Technology".to_string(), "152.81023047505408".to_string(), "153.42".to_string(), "50.21386293173349".to_string()],
    ["AMZN".to_string(), "159.23".to_string(), "0.83".to_string(), "3116".to_string(), "Technology".to_string(), "159.24418138211198".to_string(), "160.06".to_string(), "35.91252425704989".to_string()],
    ["FB".to_string(), "160.92".to_string(), "3.35".to_string(), "6028".to_string(), "Technology".to_string(), "161.03771752777985".to_string(), "164.26999999999998".to_string(), "47.22515057816757".to_string()],
    ["TSLA".to_string(), "141.39".to_string(), "-3.86".to_string(), "7303".to_string(), "Technology".to_string(), "145.25".to_string(), "142.2383483999015".to_string(), "44.17893918776443".to_string()],
    ["NVDA".to_string(), "155.31".to_string(), "2.88".to_string(), "7620".to_string(), "Technology".to_string(), "155.43453304043737".to_string(), "158.19".to_string(), "66.35918978942784".to_string()],
    ["INTC".to_string(), "137.27".to_string(), "-0.78".to_string(), "3832".to_string(), "Technology".to_string(), "138.05".to_string(), "138.36361328722475".to_string(), "64.47198235420694".to_string()],
    ["AMD".to_string(), "166.86".to_string(), "-2.45".to_string(), "4838".to_string(), "Technology".to_string(), "169.31".to_string(), "167.89351712456903".to_string(), "61.340558393713266".to_string()],
    ["ORCL".to_string(), "147.17".to_string(), "-1.44".to_string(), "8633".to_string(), "Technology".to_string(), "148.60999999999999".to_string(), "148.23402096933813".to_string(), "63.68619435204289".to_string()],
    ["DE".to_string(), "108.57".to_string(), "-4.94".to_string(), "7684".to_string(), "Agriculture".to_string(), "113.50999999999999".to_string(), "109.34604069344562".to_string(), "64.7744495022256".to_string()],
    ["AGCO".to_string(), "191.51".to_string(), "4.65".to_string(), "905".to_string(), "Agriculture".to_string(), "191.79683845810504".to_string(), "196.16".to_string(), "35.309523809266054".to_string()],
    ["CF".to_string(), "188.84".to_string(), "-2.29".to_string(), "1951".to_string(), "Agriculture".to_string(), "191.13".to_string(), "189.7788698173305".to_string(), "42.010296697291146".to_string()],
    ["MOS".to_string(), "176.73".to_string(), "4.83".to_string(), "4774".to_string(), "Agriculture".to_string(), "176.83713671007948".to_string(), "181.56".to_string(), "59.4336409923246".to_string()],
    ["ADM".to_string(), "126.9".to_string(), "2.0".to_string(), "7385".to_string(), "Agriculture".to_string(), "126.99305223664453".to_string(), "128.9".to_string(), "65.96470736777891".to_string()],
    ["BG".to_string(), "101.23".to_string(), "4.84".to_string(), "8449".to_string(), "Agriculture".to_string(), "101.3471385655594".to_string(), "106.07000000000001".to_string(), "52.59723901605819".to_string()],
    ["CTVA".to_string(), "187.94".to_string(), "3.45".to_string(), "6030".to_string(), "Agriculture".to_string(), "188.23396687668375".to_string(), "191.39".to_string(), "52.56677730034326".to_string()],
    ["FMC".to_string(), "181.29".to_string(), "3.8".to_string(), "2190".to_string(), "Agriculture".to_string(), "181.46009629618857".to_string(), "185.09".to_string(), "60.89458921426792".to_string()],
    ["SMG".to_string(), "135.93".to_string(), "4.73".to_string(), "2864".to_string(), "Agriculture".to_string(), "136.3465604747488".to_string(), "140.66".to_string(), "57.095232381557544".to_string()],
    ["NTR".to_string(), "157.23".to_string(), "2.22".to_string(), "9510".to_string(), "Agriculture".to_string(), "157.31312276514103".to_string(), "159.45".to_string(), "58.92483935867831".to_string()],
    ["GFF".to_string(), "107.01".to_string(), "0.1".to_string(), "1530".to_string(), "Fishing".to_string(), "107.0563201497178".to_string(), "107.11".to_string(), "63.59228329409604".to_string()],
    ["AQUA".to_string(), "122.47".to_string(), "4.15".to_string(), "5405".to_string(), "Fishing".to_string(), "122.53304176611964".to_string(), "126.62".to_string(), "34.15372927246516".to_string()],
    ["CQB".to_string(), "171.82".to_string(), "-3.48".to_string(), "1536".to_string(), "Fishing".to_string(), "175.29999999999998".to_string(), "172.7665665790552".to_string(), "38.29340004598403".to_string()],
    ["LZB".to_string(), "115.04".to_string(), "-0.84".to_string(), "9753".to_string(), "Fishing".to_string(), "115.88000000000001".to_string(), "115.841672337127".to_string(), "46.565909880315544".to_string()],
    ["CPG".to_string(), "146.97".to_string(), "-2.13".to_string(), "7245".to_string(), "Fishing".to_string(), "149.1".to_string(), "147.83383425679193".to_string(), "43.7698638885219".to_string()],
    ["HAR".to_string(), "183.96".to_string(), "4.53".to_string(), "7197".to_string(), "Fishing".to_string(), "184.35231299782757".to_string(), "188.49".to_string(), "59.63294941119373".to_string()],
    ["FISH".to_string(), "121.32".to_string(), "3.23".to_string(), "520".to_string(), "Fishing".to_string(), "121.87043441533523".to_string(), "124.55".to_string(), "42.73034759106266".to_string()],
    ["SEA".to_string(), "186.62".to_string(), "-0.25".to_string(), "824".to_string(), "Fishing".to_string(), "186.87".to_string(), "187.63279767827245".to_string(), "40.86118635459094".to_string()],
    ["OCEAN".to_string(), "116.85".to_string(), "-3.6".to_string(), "2264".to_string(), "Fishing".to_string(), "120.44999999999999".to_string(), "117.51856070808583".to_string(), "35.80825627424613".to_string()],
    ["WAVE".to_string(), "167.68".to_string(), "0.93".to_string(), "2316".to_string(), "Fishing".to_string(), "167.7948965901536".to_string(), "168.61".to_string(), "35.942687785714334".to_string()],
    ["NKE".to_string(), "103.31".to_string(), "-4.09".to_string(), "5105".to_string(), "Textile".to_string(), "107.4".to_string(), "104.05572167727648".to_string(), "52.83872709413055".to_string()],
    ["ADDYY".to_string(), "183.75".to_string(), "-4.17".to_string(), "6903".to_string(), "Textile".to_string(), "187.92".to_string(), "184.74107622629765".to_string(), "46.37362066796251".to_string()],
    ["PVH".to_string(), "167.95".to_string(), "2.36".to_string(), "9220".to_string(), "Textile".to_string(), "168.3686641304841".to_string(), "170.31".to_string(), "57.20609867898759".to_string()],
    ["RL".to_string(), "150.2".to_string(), "0.19".to_string(), "3457".to_string(), "Textile".to_string(), "150.35005004112338".to_string(), "150.39".to_string(), "45.081691092033324".to_string()],
    ["UAA".to_string(), "156.98".to_string(), "-4.8".to_string(), "2616".to_string(), "Textile".to_string(), "161.78".to_string(), "157.84344189262464".to_string(), "59.29822366036095".to_string()],
    ["VFC".to_string(), "191.33".to_string(), "3.72".to_string(), "3758".to_string(), "Textile".to_string(), "191.33740911398263".to_string(), "195.05".to_string(), "67.19604365764681".to_string()],
    ["HBI".to_string(), "189.66".to_string(), "4.53".to_string(), "2118".to_string(), "Textile".to_string(), "189.80723762812033".to_string(), "194.19".to_string(), "48.8067197445065".to_string()],
    ["GIL".to_string(), "153.79".to_string(), "0.86".to_string(), "1662".to_string(), "Textile".to_string(), "154.2217874788167".to_string(), "154.65".to_string(), "59.23621666228738".to_string()],
    ["LULU".to_string(), "165.47".to_string(), "-2.62".to_string(), "4860".to_string(), "Textile".to_string(), "168.09".to_string(), "166.4966943960409".to_string(), "64.53978503791065".to_string()],
    ["LEVI".to_string(), "134.25".to_string(), "2.13".to_string(), "7805".to_string(), "Textile".to_string(), "134.3574506953376".to_string(), "136.38".to_string(), "39.86405964229729".to_string()],
    ["WMT".to_string(), "171.34".to_string(), "0.52".to_string(), "9374".to_string(), "Sales".to_string(), "171.45002583594518".to_string(), "171.86".to_string(), "57.427088491768544".to_string()],
    ["COST".to_string(), "198.47".to_string(), "-1.41".to_string(), "3069".to_string(), "Sales".to_string(), "199.88".to_string(), "199.5933590670079".to_string(), "45.578277621243636".to_string()],
    ["TGT".to_string(), "173.76".to_string(), "-2.46".to_string(), "2967".to_string(), "Sales".to_string(), "176.22".to_string(), "174.53321755007272".to_string(), "37.393349255987815".to_string()],
    ["LOW".to_string(), "172.11".to_string(), "-0.97".to_string(), "7007".to_string(), "Sales".to_string(), "173.08".to_string(), "173.14115840907272".to_string(), "43.94138540698927".to_string()],
    ["HD".to_string(), "192.17".to_string(), "2.39".to_string(), "8737".to_string(), "Sales".to_string(), "192.2799920578784".to_string(), "194.55999999999997".to_string(), "57.995919785530134".to_string()],
    ["BBY".to_string(), "107.06".to_string(), "-0.24".to_string(), "763".to_string(), "Sales".to_string(), "107.3".to_string(), "107.97374060513775".to_string(), "66.79921719029369".to_string()],
    ["M".to_string(), "152.14".to_string(), "0.43".to_string(), "5561".to_string(), "Sales".to_string(), "152.16107984615184".to_string(), "152.57".to_string(), "48.92579951157485".to_string()],
    ["JWN".to_string(), "145.5".to_string(), "3.62".to_string(), "5437".to_string(), "Sales".to_string(), "145.62523280332917".to_string(), "149.12".to_string(), "44.910619097708164".to_string()],
    ["KSS".to_string(), "137.05".to_string(), "-4.99".to_string(), "4899".to_string(), "Sales".to_string(), "142.04000000000002".to_string(), "138.0276803036662".to_string(), "40.667002273066416".to_string()],
    ["DG".to_string(), "141.49".to_string(), "-3.17".to_string(), "2757".to_string(), "Sales".to_string(), "144.66".to_string(), "142.4668786066943".to_string(), "31.89539689158398".to_string()],
    ["WBA".to_string(), "161.52".to_string(), "-1.8".to_string(), "9911".to_string(), "Food and Beverage Retail".to_string(), "163.32000000000002".to_string(), "162.45909338919805".to_string(), "65.90832532751895".to_string()],
    ["CVS".to_string(), "107.8".to_string(), "-3.05".to_string(), "6097".to_string(), "Food and Beverage Retail".to_string(), "110.85".to_string(), "108.41705031423173".to_string(), "47.65700537474435".to_string()],
    ["KR".to_string(), "195.6".to_string(), "0.59".to_string(), "2526".to_string(), "Food and Beverage Retail".to_string(), "195.8356786581809".to_string(), "196.19".to_string(), "65.9713112296831".to_string()],
    ["SYY".to_string(), "173.55".to_string(), "2.97".to_string(), "661".to_string(), "Food and Beverage Retail".to_string(), "173.89687142613818".to_string(), "176.52".to_string(), "42.30192635601739".to_string()],
    ["BUD".to_string(), "187.22".to_string(), "-3.41".to_string(), "7092".to_string(), "Food and Beverage Retail".to_string(), "190.63".to_string(), "188.39303268723472".to_string(), "61.798080405436394".to_string()],
    ["TAP".to_string(), "127.07".to_string(), "3.94".to_string(), "6507".to_string(), "Food and Beverage Retail".to_string(), "127.4386319910922".to_string(), "131.01".to_string(), "38.23570447772705".to_string()],
    ["KO".to_string(), "120.68".to_string(), "-3.12".to_string(), "9734".to_string(), "Food and Beverage Retail".to_string(), "123.80000000000001".to_string(), "121.68238747986264".to_string(), "52.45467900385535".to_string()],
    ["PEP".to_string(), "185.66".to_string(), "2.39".to_string(), "9321".to_string(), "Food and Beverage Retail".to_string(), "186.0689024501597".to_string(), "188.04999999999998".to_string(), "54.605560605480065".to_string()],
    ["GIS".to_string(), "163.43".to_string(), "2.31".to_string(), "9037".to_string(), "Food and Beverage Retail".to_string(), "163.7871853565068".to_string(), "165.74".to_string(), "30.351650314385893".to_string()],
    ["K".to_string(), "189.03".to_string(), "-1.81".to_string(), "9016".to_string(), "Food and Beverage Retail".to_string(), "190.84".to_string(), "189.77513559577352".to_string(), "52.41739767580344".to_string()]
    ];

    //STOCK PREDICTION PARAMETERS
    // Stock prediction brownian motion model parameters (columns = drift coefficient, votality coefficient, weiner process)
    let mut parameters = vec![
    ["AAPL","0.0004370861069626262","0.01777354579378964","-0.7020530938773524"],
    ["MSFT","0.0009556428757689245","0.015426980635477917","-0.3276621465977682"],
    ["GOOGL","0.0007587945476302646","0.026574750183038585","-0.39210815313215763"],
    ["AMZN","0.000638792635777333","0.017135066533871784","-1.4635149481321186"],
    ["FB","0.00024041677639819288","0.015618690193747614","0.29612027706457605"],
    ["TSLA","0.00024039506830258236","0.02085392166316497","0.26105527217988933"],
    ["NVDA","0.00015227525095137952","0.012818484499495252","0.00511345664246089"],
    ["INTC","0.0008795585311974417","0.026043939615080794","-0.23458713337514692"],
    ["AMD","0.000641003510568888","0.011491012873595417","-1.4153707420504142"],
    ["ORCL","0.0007372653200164409","0.02973773873201034","-0.42064532276535904"],
    ["DE","0.00011852604486622221","0.02544489538593315","-0.3427145165267695"],
    ["AGCO","0.0009729188669457949","0.013974313630683449","-0.8022772692216189"],
    ["CF","0.0008491983767203796","0.010110442342472048","-0.16128571166600914"],
    ["MOS","0.00029110519961044856","0.026309228569096683","0.4040508568145384"],
    ["ADM","0.00026364247048639054","0.024137146876952342","1.8861859012105302"],
    ["BG","0.00026506405886809044","0.024580143360819744","0.17457781283183896"],
    ["CTVA","0.00037381801866358396","0.025425406933718912","0.25755039072276437"],
    ["FMC","0.000572280788469014","0.011480893034681807","-0.07444591576616721"],
    ["SMG","0.0004887505167779041","0.01716931457088545","-1.9187712152990415"],
    ["NTR","0.0003621062261782377","0.012317381190502595","-0.026513875449216878"],
    ["GFF","0.0006506676052501415","0.027262068517511867","0.06023020994102644"],
    ["AQUA","0.00022554447458683765","0.022465962536551157","2.463242112485286"],
    ["CQB","0.00036293018368169634","0.016617960497052983","-0.19236096478112252"],
    ["LZB","0.0004297256589643225","0.011271167005720473","0.30154734233361247"],
    ["CPG","0.0005104629857953323","0.016219646434313242","-0.03471176970524331"],
    ["HAR","0.0008066583652537123","0.016503666440534942","-1.168678037619532"],
    ["FISH","0.00027970640394252375","0.024592123566761277","1.1428228145150205"],
    ["SEA","0.0005628109945722505","0.02275114942710426","0.7519330326867741"],
    ["OCEAN","0.0006331731119758382","0.027744254851526526","0.7910319470430469"],
    ["WAVE","0.00014180537144799796","0.019444298503238984","-0.9093874547947389"],
    ["NKE","0.0006467903667112945","0.012391884918766034","1.4027943109360992"],
    ["ADDYY","0.0002534717113185624","0.024264895744459898","-1.4018510627922809"],
    ["PVH","0.00015854643368675157","0.025215700972337947","0.5868570938002703"],
    ["RL","0.000953996983528","0.021225543951389925","2.1904556258099785"],
    ["UAA","0.0009690688297671035","0.025419343599091218","-0.9905363251306883"],
    ["VFC","0.0008275576133048151","0.019875911927287812","-0.5662977296027719"],
    ["HBI","0.0003741523922560336","0.02045465658763988","0.09965136508764122"],
    ["GIL","0.00018790490260574548","0.01855082036717099","-0.5034756541161992"],
    ["LULU","0.0006467903667112945","0.012391884918766034","1.4027943109360992"],
    ["LEVI","0.0004961372443656412","0.012157828539866088","0.06856297480602733"],
    ["WMT","0.00020983441136030096","0.010628583713734685","-1.0623037137261049"],
    ["COST","0.0005456592191001432","0.022728208225275607","0.4735924306351816"],
    ["TGT","0.00013094966900369657","0.016287119621526534","-0.9194242342338032"],
    ["LOW","0.0009183883618709039","0.020171413823294054","1.5499344050175394"],
    ["HD","0.0003329019834400152","0.028151329478521857","-0.7832532923362371"],
    ["BBY","0.0006962700559185838","0.014985844582977498","-0.3220615162056756"],
    ["M","0.00038053996848046986","0.018207658460712595","0.8135172173696698"],
    ["JWN","0.0005680612190600297","0.02511102277086097","-1.2308643164339552"],
    ["KSS","0.0005920392514089517","0.014575963309832449","0.22745993460412942"],
    ["DG","0.00026636900997297435","0.01153959819657586","1.307142754282428"],
    ["WBA","0.0009726261649881027","0.01579502905827536","-1.6074832345612275"],
    ["CVS","0.0007976195410250031","0.013224425745080088","0.1846338585323042"],
    ["KR","0.0009455490474077703","0.02859395304685146","0.25988279424842353"],
    ["SYY","0.000905344615384884","0.026162407591288336","0.7818228717773104"],
    ["BUD","0.0006381099809299767","0.02266807513020847","-1.236950710878082"],
    ["TAP","0.0009296868115208052","0.02742921180375435","-1.3204566130842763"],
    ["KO","0.00017964325184672756","0.026073441537982286","0.5219415656168976"],
    ["PEP","0.00027638457617723067","0.013731401177720717","0.29698467323318606"],
    ["GIS","0.00014070456001948425","0.02785117996979955","0.25049285034587654"],
    ["K","0.0003927972976869379","0.020786844838313015","0.3464482094969757"]
    ];
    

    //RW LOCKS
    // create RWlocks for parameters and stocks
    let stocks = Arc::new(RwLock::new(stocks));
    let parameters = Arc::new(RwLock::new(parameters));

    //STOCK EXCHANGE THREAD
    // clone stocks and parameters for thread
    let stocks_for_thread = Arc::clone(&stocks);
    let parameters_for_thread = Arc::clone(&parameters);

    //stock exchange (Bursa) logic 
    let StockExchangeUpdate = thread::spawn(move ||{
        loop{
            thread::sleep(Duration::from_secs(1));
            for i in 0..60{
                let mut new_price = 0.0;
                let mut new_sr = 0.0;
                let mut new_br = 0.0;
                let mut s0f = 0.0;
                {
                    let stocks = stocks_for_thread.read().unwrap();
                    let parameters = parameters_for_thread.read().unwrap();
                    
                    let initial_price = &stocks[i][1];
                    let s0 = initial_price;  // Initial stock price
                    let sr = &stocks[i][5]; // selling rate
                    let br = &stocks[i][6]; //buying rate
                    let rsi = &stocks[i][7]; //RSI

                    let mu = parameters[i][1];   // Drift coefficient (annual return)
                    let sigma = parameters[i][2]; // Volatility coefficient (annual volatility)
                    let t = 1.0 / 365.0; // Time period (1 day)
                    
                    // Generate a random value from normal distribution
                    let z = generate_norm_rand(); //random number for current price
                    let z_sr = generate_norm_rand(); //random number for selling rate
                    let z_br = generate_norm_rand(); //random number for buying rate

                    // Calculate stock price for 1 day
                    s0f = s0.parse().expect("Failed to parse number");
                    let muf:f64 = mu.parse().expect("Failed to parse number");
                    let sigf:f64 = sigma.parse().expect("Failed to parse number");
                    let pricef:f64 = initial_price.parse().expect("Failed to parse number");

                    //Eulers constant
                    let E: f64 = 2.71828182845904523536028747135266250_f64;

                    //calculate new current price
                    let number = (muf-0.5*sigf.powf(2.0))*t+sigf*z*t.sqrt();
                    new_price = pricef*E.powf(number);

                    //calculate new selling rate 
                    let number = (muf-0.5*sigf.powf(2.0))*t+sigf*z_sr*t.sqrt();
                    new_sr = pricef*E.powf(number);
                    
                    //calculate new buying rate
                    let number = (muf-0.5*sigf.powf(2.0))*t+sigf*z_br*t.sqrt();
                    new_br = pricef*E.powf(number);
                }
                
                // Edit the price, selling rate,buying rate,RSI and change after 1 day in stocks
                let mut stocks = stocks_for_thread.write().unwrap();
                stocks[i][1] = new_price.to_string(); // update current price
                stocks[i][5] = new_sr.to_string(); // update selling rate
                stocks[i][6] = new_br.to_string(); // update buying rate
                stocks[i][7] = generate_rand_rsi().to_string(); // update rsi
                stocks[i][2] = (((new_price-s0f)/s0f)*100.0).to_string(); // update change in 1 day
            }    
            // testing prints            
            let stocks = stocks_for_thread.read().unwrap();
            println!("AAPL {:?}",&stocks[0]);
            println!("MSFT {:?}",&stocks[1]);
            println!("GOOG {:?}",&stocks[2]);
            println!("AMZN {:?}",&stocks[3]);
            println!("FB {:?}",&stocks[4]);
            println!("TSLA {:?}",&stocks[5]);
            println!("   ");
        }
    });

    //USER
    struct UserPortfolio {
        name: String,
        sector: String,
        buy_rsi: f64,
        sell_rsi: f64,
        rate_change: f64,
        buy_trading_profit: f64,
        sell_trading_profit: f64,
        sell_loss: i32,
        funds: f64,
        safety_funds: f64,
        max_rate: f64,
    }

    //TRANSACTION
    struct StockTransaction {
        name:String,
        stock_name: String,
        user_funds: f64,
        stock_price: f64,
        buy_sell: String,
    }

    // channel to send buy/sell requests to broker
    let (tx, rx) = mpsc::channel::<StockTransaction>();
    let tx = Arc::new(Mutex::new(tx));
    
    //channel for broker to respond to user1
    let (tx1, rx1) = mpsc::channel::<String>();
    let tx1 = Arc::new(Mutex::new(tx1));

    //EVENT PROCESSING ENGINE

    let stocks_for_thread = Arc::clone(&stocks);
    let tx_for_thread = Arc::clone(&tx);
    let EventProcessingEngine = thread::spawn(move ||{
        // user criteria
        let mut user = UserPortfolio {
            name: "Khabib Nurmagamedov".to_string(),
            sector: "Technology".to_string(),
            buy_rsi: 70.0,
            sell_rsi: 30.0,
            rate_change: 0.05,
            buy_trading_profit: 0.0,
            sell_trading_profit: 10.0,
            sell_loss: 5,
            funds: 90000.0,
            safety_funds: 10000.0,
            max_rate: 160.0,
        };

        // stocks bought (cols=symbol,numOfStocks,priceBought)
        let mut stocks_bought = vec![
            ["AAPL".to_string(),"0".to_string(),"0".to_string()],
            ["MSFT".to_string(),"0".to_string(),"0".to_string()],
            ["GOOGL".to_string(),"0".to_string(),"0".to_string()],
            ["AMZN".to_string(),"0".to_string(),"0".to_string()],
            ["FB".to_string(),"0".to_string(),"0".to_string()], 
            ["TSLA".to_string(),"0".to_string(),"0".to_string()],
            ["NVDA".to_string(),"0".to_string(),"0".to_string()],
            ["INTC".to_string(),"0".to_string(),"0".to_string()],
            ["AMD".to_string(),"0".to_string(),"0".to_string()],
            ["ORCL".to_string(),"0".to_string(),"0".to_string()],
        ];

        //monitor and buy/sell stocks accordingly
        loop{
            thread::sleep(Duration::from_secs(2));
            // variables will store information needed to modify stock quantity as this cannot be done in loop due to read lock

            let mut stock_name_to_mod = "".to_string();
            let mut stock_quantity_to_mod = 0;
            
            
            // read stocks
            {
                let stocks = stocks_for_thread.read().unwrap(); 
                for stock in stocks.iter(){
                    let s_sector = &stock[4];
                    let s_current_price:f64 = stock[1].parse().expect("Error converting string to float");
                    let s_rsi:f64 =  stock[7].parse().expect("Error converting string to float");
                    let s_change:f64 =  stock[2].parse().expect("Error converting string to float");
                    let no_stocks:i64 = stock[3].parse().expect("Error converting string to float");
                    let s_br:f64 = stock[6].parse().expect("Error converting string to float");

                    if user.sector==*s_sector && s_current_price<user.max_rate && s_rsi<user.sell_rsi 
                    && s_change>user.rate_change{
                        
                        // send the purchase request to the channel
                        let transaction = StockTransaction{
                            name:"1".to_string(),
                            stock_name: stock[0].clone(),
                            user_funds: user.funds-user.safety_funds,
                            stock_price: s_current_price,
                            buy_sell: "buy".to_string(),
                        };

                        println!("{} has requested the broker to purchase {} stocks worth {:?}",user.name,stock[0],user.funds-user.safety_funds);
                        //send the transaction to the broker
                        {
                            let tx_locked = tx_for_thread.lock().unwrap();
                            tx_locked.send(transaction).unwrap();
                        }

                        //wait for the response of the broker
                        if let Ok(response) = rx1.try_recv(){
                            if response=="Accepted".to_string(){
                                // calculate stocks bought
                                let max_purchase = user.funds as f64 -user.safety_funds as f64; // the current buying capacity of the user
                                let mut num_stocks_purchased = 0;
                                if max_purchase<(no_stocks as f64 * s_br){
                                    let val = max_purchase/s_br;
                                    num_stocks_purchased = val as i64;
                                }else{
                                    num_stocks_purchased = no_stocks;
                                }
    
                                // deduct number of stocks from stock market
                                stock_name_to_mod = stock[0].clone();
                                stock_quantity_to_mod = num_stocks_purchased;
    
                                // add number of stocks and buy rate to stocks bought 
                                for sb in stocks_bought.iter_mut(){
                                    if sb[0]==stock[0]{
                                        sb[1] = num_stocks_purchased.to_string();
                                        sb[2] = s_br.to_string();
                                    }
                                }
    
                                // update user funds
                                for sb in stocks_bought.iter() {
                                    if sb[0] == stock[0] {
                                        let num_shares: f64 = sb[1].parse().unwrap();
                                        let price_per_share: f64 = sb[2].parse().unwrap();
                                        user.funds -= num_shares * price_per_share;
                                    }
                                }
    
                                if num_stocks_purchased!=0{
                                    println!("{} has bought {:?} {} stocks and has {:?} remaining now",user.name,num_stocks_purchased,stock[0],user.funds-user.safety_funds);
                                }
                            }else{
                                println!("{} request to buy {} stocks has been rejected",user.name,stock[0]);
                            }
                        }
                    } 
                }
                
            }
            
            let mut stocks = stocks_for_thread.write().unwrap();
            for stock in stocks.iter_mut() {
                if stock[0] == stock_name_to_mod {
                    let current_quantity: i32 = stock[3].parse().expect("Error converting string to i32");
                    let new_quantity = current_quantity - stock_quantity_to_mod as i32;
                    stock[3] = new_quantity.to_string();
                    break; // only one entry per stock, we can exit the loop once the match is found
                }
            }
            // drop the lock
            drop(stocks);


            // sell stocks:
            let mut num_stocks = 0;
            let mut stock_name_to_mod = "".to_string();
            // read lock
            let stocks = stocks_for_thread.read().unwrap(); 
            
            // monitor and sell stocks
            for stock in stocks.iter(){
                for sb in stocks_bought.iter_mut(){
                    if sb[0]==stock[0] && sb[1]!= "0".to_string(){
                        
                            // calculate profit or loss
                            let symbol = &stock[0];
                            let shares_bought: f64 = sb[1].parse().unwrap();
                            let share_value_when_buying: f64 = sb[2].parse().unwrap();
                            let current_price: f64 = stock[1].parse().unwrap();

                            let profit = (current_price - share_value_when_buying) * shares_bought;
                            let profit_loss_percentage = ((current_price - share_value_when_buying) / share_value_when_buying) * 100.0;
                            let s_rsi:f64 =  stock[7].parse().expect("Error converting string to float");
                            let s_sr:f64 =  stock[5].parse().expect("Error converting string to float");

                            if s_rsi>user.buy_rsi || profit_loss_percentage>0.1 || profit_loss_percentage< -0.05{

                                // send the purchase request to the channel
                                let transaction = StockTransaction{
                                    name:"1".to_string(),
                                    stock_name: stock[0].clone(),
                                    user_funds: user.funds-user.safety_funds,
                                    stock_price: stock[1].parse().expect("Error converting"),
                                    buy_sell: "sell".to_string(),
                                };

                                println!("{} has requested the broker to sell {} stocks worth {:?}",user.name,stock[0],sb[1]);
                                //send the transaction to the broker
                                {
                                    let tx_locked = tx_for_thread.lock().unwrap();
                                    tx_locked.send(transaction).unwrap();
                                }
                                
                                // We are waiting for the broker to send a response in the meanwhile

                                 //wait for the response of the broker
                                if let Ok(response) = rx1.try_recv(){
                                    if response=="Accepted".to_string(){
                                        let num_stocks_f:f64 = sb[1].parse().expect("error converting");
                                        num_stocks = num_stocks_f as i64;
                                        let new_funds = num_stocks as f64 *s_sr;
                                        stock_name_to_mod = stock[0].clone();
                                        // update user funds
                                        user.funds += new_funds;

                                        // clear the stock from stocks_owned variable
                                        sb[1] = "0".to_string();
                                        sb[2] = "0".to_string();
                                        
                                        if num_stocks!=0{
                                            println!("{} has sold {:?} of {} stock and now has {:?} ringitt with a profit of {}",user.name,num_stocks,stock[0],user.funds,profit);
                                        }
                                    }
                                }

                            }        
                    }
                }
            }

            drop(stocks);

            // update stock quantity
            let mut stocks = stocks_for_thread.write().unwrap();
            for stock in stocks.iter_mut() {
                if stock[0] == stock_name_to_mod {
                    let current_quantity: i32 = stock[3].parse().expect("Error converting string to i32");
                    let new_quantity = current_quantity + stock_quantity_to_mod as i32;
                    stock[3] = new_quantity.to_string();
                    break; // only one entry per stock, we can exit the loop once the match is found
                }
            }
        }
    });



    //BROKER:
    let tx_for_thread = Arc::clone(&tx);
    let tx1_for_thread = Arc::clone(&tx1);
    let parameters_for_thread = Arc::clone(&parameters);
    let stocks_for_thread = Arc::clone(&stocks);

    let Broker = thread::spawn(move ||{
        loop{
            thread::sleep(Duration::from_millis(500));
            if let Ok(transaction) = rx.try_recv(){
                // decide order type based on the votality of the stock
                let mut limitOrder=false;
                {
                    let parameters = parameters_for_thread.read().unwrap();
                    for param in parameters.iter(){
                        if param[0]==transaction.stock_name{
                            let volatility:f64 = param[2].parse().expect("conversion error");
                            if volatility<0.2{
                                limitOrder=false;
                            }else{
                                limitOrder=true;
                            }
                        }
                    }
                }
                

                // Market Order
                if !limitOrder{
                    if transaction.name == "1".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Khabib Nurmagamedov as a Market order",transaction.buy_sell);
                    }
                    if transaction.name == "2".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Conor McGregor as a Market order",transaction.buy_sell);
                    }
                    if transaction.name == "3".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Jon Jones as a Market order",transaction.buy_sell);
                    }
                }

                // Limit Order (takes longer to execute than a market order)
                if limitOrder{
                    thread::sleep(Duration::from_millis(2000));
                    if transaction.name == "1".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Khabib Nurmagamedov as a Limit order",transaction.buy_sell);
                    }
                    if transaction.name == "2".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Conor McGregor as a Limit order",transaction.buy_sell);
                    }
                    if transaction.name == "3".to_string(){
                        let tx1_locked = tx1_for_thread.lock().unwrap();
                        tx1_locked.send("Accepted".to_string()).unwrap();
                        println!("Broker has accepted {} order for Jon Jones as a Limit order",transaction.buy_sell);
                    }
                }
            }
    }

    });


    StockExchangeUpdate.join().unwrap();
    EventProcessingEngine.join().unwrap();
    Broker.join().unwrap();

}
