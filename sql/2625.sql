select CONCAT(SUBSTRING(np.cpf,1,3),'.',
              SUBSTRING(np.cpf,4,3),'.',
              SUBSTRING(np.cpf,7,3),'-',
              SUBSTRING(np.cpf,10,2)) as cpf from natural_person as np