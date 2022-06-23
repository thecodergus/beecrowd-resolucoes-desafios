select pd.name as name, pv.name as name 
from products as pd inner join providers as pv on pd.id_providers = pv.id 
where pd.id_categories = 6;