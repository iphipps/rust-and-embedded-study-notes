# Setting up a development environment


## Documentation

These manual links will be useful.
(STM32F3DISCOVERY User Manual)[http://www.st.com/resource/en/user_manual/dm00063382.pdf]
(STM32F303VC Datasheet)[http://www.st.com/resource/en/datasheet/stm32f303vc.pdf]
(STM32F303VC Reference Manual)[http://www.st.com/resource/en/reference_manual/dm00043574.pdf]
(LSM303DLHC)[http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf]
(L3GD20)[http://www.st.com/resource/en/datasheet/l3gd20.pdf]


## Tools

on a mac run ./02_install_tools.sh
to install everything.  You'll need to have rust, cargo and brew installed already.

# Confirm it works by running

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```


